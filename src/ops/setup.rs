use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn setup(config: Config) -> Result<(), String> {
    let target = PathBuf::from(config.general.target_dir);
    let input = target.clone().join("out").into_os_string();

    let vk_path = target.clone().join("verification.key").into_os_string();

    let pk_path = target.clone().join("proving.key").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let vk_path = Argument::new("-v", Some(vk_path.to_str().unwrap()));
    let pk_path = Argument::new("-p", Some(pk_path.to_str().unwrap()));
    let proving_scheme = Argument::new("-s", Some(config.crypto.proving_scheme.as_str()));

    let cmd = Command::new("setup", vec![input, vk_path, pk_path, proving_scheme]);
    Executor::execute(cmd)
}
