use crate::zpm_core::executor::{Argument, Command, Executor};
use crate::zpm_core::Config;
use std::path::PathBuf;

pub struct CompileCommand {}

impl CompileCommand {
    pub fn run(config: Config) -> Result<(), String> {
        let input = PathBuf::from(config.general.source_dir)
            .join(config.general.entry)
            .into_os_string()
            .into_string()
            .unwrap();

        let output = PathBuf::from(config.general.target_dir)
            .join("out")
            .into_os_string()
            .into_string()
            .unwrap();

        let input = Argument::new("--input", input.as_str());
        let output = Argument::new("--output", output.as_str());

        let cmd = Command::new("compile", vec![input, output]);
        Executor::execute(cmd)
    }
}
