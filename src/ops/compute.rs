use crate::core::constants::DEFAULT_TARGET_DIR;
use crate::core::executor::{Argument, Command, Executor};
use clap::ArgMatches;
use std::path::PathBuf;

pub fn compute(matches: &ArgMatches) -> Result<(), String> {
    let target = PathBuf::from(DEFAULT_TARGET_DIR);

    let input = target.clone().join("out").into_os_string();
    let output = target.clone().join("witness").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let output = Argument::new("-o", Some(output.to_str().unwrap()));

    match matches.is_present("stdin") {
        true => {
            let abi_spec = target.clone().join("abi.json").into_os_string();

            let stdin = Argument::new("--stdin", None);
            let abi = Argument::new("--abi", None);
            let abi_spec = Argument::new("-s", Some(abi_spec.to_str().unwrap()));

            let cmd = Command::new("compute-witness", vec![input, output, abi_spec, abi, stdin]);
            Executor::execute(cmd, true)
        }
        false => {
            if matches.is_present("arguments") {
                let args: Vec<&str> = matches.values_of("arguments").unwrap().collect();
                let arguments_flag = Argument::new("-a", None);
                let mut arguments = vec![input, output, arguments_flag];
                for x in args {
                    arguments.push(Argument::new(x, None));
                }
                let cmd = Command::new("compute-witness", arguments);
                Executor::execute(cmd, false)
            } else {
                let cmd = Command::new("compute-witness", vec![input, output]);
                Executor::execute(cmd, false)
            }
        }
    }
}
