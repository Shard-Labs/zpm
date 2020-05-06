use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn compile(config: Config) -> Result<(), String> {
    let input = PathBuf::from(config.general.source_dir)
        .join(config.general.entry)
        .into_os_string();

    let target = PathBuf::from(config.general.target_dir);

    let output = target.clone().join("out").into_os_string();

    let abi_spec = target.clone().join("abi.json").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let output = Argument::new("-o", Some(output.to_str().unwrap()));
    let abi_spec = Argument::new("-s", Some(abi_spec.to_str().unwrap()));

    let cmd = Command::new("compile", vec![input, output, abi_spec]);
    Executor::execute(cmd)
}
