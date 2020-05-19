use crate::core::constants::DEFAULT_TARGET_DIR;
use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn setup(config: Config) -> Result<(), String> {
    let target = PathBuf::from(DEFAULT_TARGET_DIR);
    let input = target.clone().join("out").into_os_string();

    let vk_path = target.clone().join("verification.key").into_os_string();

    let pk_path = target.clone().join("proving.key").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let vk_path = Argument::new("-v", Some(vk_path.to_str().unwrap()));
    let pk_path = Argument::new("-p", Some(pk_path.to_str().unwrap()));

    let backend = Argument::new("-b", Some(config.crypto.backend.as_str()));
    let proving_scheme = Argument::new("-s", Some(config.crypto.proving_scheme.as_str()));

    let cmd = Command::new("setup", vec![input, vk_path, pk_path, backend, proving_scheme]);
    Executor::execute(cmd, false)
}
