use crate::AnyError;

use crate::manifest::*;

use std::path::Path;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct LinuxkitMake {
    #[structopt(default_value = ".")]
    pub project_dir: String,
}

impl LinuxkitMake {
    pub fn run(&self) -> Result<(), AnyError> {
        let project_root: &Path = self.project_dir.as_str().as_ref();

        let manifest = Manifest {
            kernel: util::read_file(project_root.join("kernel.yml"))?,
            init: util::read_dir_as_vec(project_root.join("init.d"))?,
            on_boot: util::read_dir_as_vec(project_root.join("on-boot.d"))?,
            on_shutdown: util::read_dir_as_vec(project_root.join("on-shutdown.d"))?,
            services: util::read_dir_as_vec(project_root.join("services.d"))?,
            files: util::read_dir_as_vec(project_root.join("files.d"))?,
            trust: util::read_file(project_root.join("trust.yml"))?,
        };

        let stdout = std::io::stdout();
        let stdout = stdout.lock();
        let () = ::serde_yaml::to_writer(stdout, &manifest)?;

        Ok(())
    }
}

mod util {
    use crate::AnyError;
    use ::serde::de::DeserializeOwned;
    use std::fs::OpenOptions;
    use std::path::Path;

    pub fn read_file<T, P>(path: P) -> Result<T, AnyError>
    where
        P: AsRef<Path>,
        T: DeserializeOwned,
    {
        let f = OpenOptions::new().read(true).open(path)?;
        let v = ::serde_yaml::from_reader(f)?;
        Ok(v)
    }

    pub fn read_dir_as_vec<T, P>(path: P) -> Result<Vec<T>, AnyError>
    where
        P: AsRef<Path>,
        T: DeserializeOwned,
    {
        let dir_entries = std::fs::read_dir(path)?;

        let mut items = vec![];

        for dir_entry in dir_entries {
            let dir_entry = dir_entry?;

            let file_name = dir_entry.file_name().to_string_lossy().as_ref().to_owned();

            if !dir_entry.file_type()?.is_file() {
                continue;
            }

            if !file_name.ends_with(".yml") {
                continue;
            }

            let value = read_file::<T, _>(dir_entry.path())?;

            items.push((file_name, value));
        }

        let () = items.sort_by_key(|(n, _)| n.to_owned());

        Ok(items.into_iter().map(|(_, v)| v).collect())
    }
}
