use ::serde_yaml::Value as JsValue;

mod kernel_spec;
pub use kernel_spec::KernelSpec;

mod init_entry;
pub use init_entry::InitEntry;

mod exec_entry;
pub use exec_entry::ExecEntry;

mod file_entry;
pub use file_entry::FileEntry;

use crate::EnvSubst;

#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct Manifest {
    #[serde(rename = "kernel")]
    pub kernel: KernelSpec,

    #[serde(rename = "init")]
    pub init: Vec<InitEntry>,

    #[serde(rename = "onboot")]
    pub on_boot: Vec<ExecEntry>,

    #[serde(rename = "onshutdown")]
    pub on_shutdown: Vec<ExecEntry>,

    #[serde(rename = "services")]
    pub services: Vec<ExecEntry>,

    #[serde(rename = "files")]
    pub files: Vec<FileEntry>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trust")]
    pub trust: Option<JsValue>,
}

impl EnvSubst for Manifest {
    fn env_subst(
        &self,
        context: &std::collections::HashMap<String, String>,
    ) -> Result<Self, envsubst::Error> {
        let kernel = self.kernel.env_subst(context)?;
        let init = self.init.env_subst(context)?;
        let on_boot = self.on_boot.env_subst(context)?;
        let on_shutdown = self.on_shutdown.env_subst(context)?;
        let services = self.services.env_subst(context)?;
        let files = self.files.env_subst(context)?;
        let trust = self.trust.env_subst(context)?;

        Ok(Self {
            kernel,
            init,
            on_boot,
            on_shutdown,
            services,
            files,
            trust,
        })
    }
}
