use ::linuxkit_utils::cli::LinuxkitMake;
use ::structopt::StructOpt;

pub fn main() {
    LinuxkitMake::from_args().run().expect("Failure")
}
