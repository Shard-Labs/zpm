use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn compute(config: Config, args: Option<Vec<&str>>) -> Result<(), String> {
    let target = PathBuf::from(config.general.target_dir);

    let input = target.clone().join("out").into_os_string();
    let output = target.clone().join("witness").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let output = Argument::new("-o", Some(output.to_str().unwrap()));

    match args {
        Some(args) => {
            let arguments_flag = Argument::new("-a", None);
            let mut arguments = vec![input, output, arguments_flag];
            for x in args {
                arguments.push(Argument::new(x, None));
            }
            let cmd = Command::new("compute-witness", arguments);
            Executor::execute(cmd)
        }
        None => {
            let cmd = Command::new("compute-witness", vec![input, output]);
            Executor::execute(cmd)
        }
    }
}
