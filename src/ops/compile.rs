use crate::core::constants::{DEFAULT_SOURCE_DIR, DEFAULT_TARGET_DIR};
use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn compile(config: Config) -> Result<(), String> {
    let input = PathBuf::from(DEFAULT_SOURCE_DIR)
        .join(config.general.entry)
        .into_os_string();

    let target = PathBuf::from(DEFAULT_TARGET_DIR);
    let output = target.clone().join("out").into_os_string();
    let abi_spec = target.clone().join("abi.json").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let output = Argument::new("-o", Some(output.to_str().unwrap()));
    let abi_spec = Argument::new("-s", Some(abi_spec.to_str().unwrap()));
    let curve = Argument::new("-c", Some(config.crypto.elliptic_curve.as_str()));

    let cmd = Command::new("compile", vec![input, output, abi_spec, curve]);
    Executor::execute(cmd, false)
}
