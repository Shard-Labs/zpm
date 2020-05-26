use crate::core::constants::DEFAULT_TARGET_DIR;
use crate::core::executor::{Argument, Command, Executor};
use clap::ArgMatches;
use std::path::PathBuf;

pub fn compute<E: Executor>(matches: &ArgMatches) -> Result<E::ExecutorResult, String> {
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

            let cmd = Command::new("compute-witness")
                .args(vec![input, output, abi_spec, abi, stdin])
                .pipe_stdin(true)
                .build();

            E::execute(cmd)
        }
        false => {
            if matches.is_present("arguments") {
                let args: Vec<&str> = matches.values_of("arguments").unwrap().collect();
                let arguments_flag = Argument::new("-a", None);
                let mut arguments = vec![input, output, arguments_flag];
                for x in args {
                    arguments.push(Argument::new(x, None));
                }
                let cmd = Command::new("compute-witness").args(arguments).build();

                E::execute(cmd)
            } else {
                let cmd = Command::new("compute-witness")
                    .args(vec![input, output])
                    .build();

                E::execute(cmd)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::executor::tests::TestingExecutor;
    use crate::create_app;
    use crate::ops::compute::compute;

    #[test]
    fn compute_with_inline_arguments() {
        let arg_vec = vec!["zpm", "compute", "-a", "1", "2", "3"];

        let app = create_app();
        let matches = app.get_matches_from_safe(arg_vec).unwrap();

        let submatches = matches.subcommand_matches("compute").unwrap();
        let cmd = compute::<TestingExecutor>(&submatches).unwrap();

        assert_eq!(
            cmd,
            "compute-witness -i target/out -o target/witness -a 1 2 3"
        )
    }

    #[test]
    fn compute_with_piped_stdin() {
        let arg_vec = vec!["zpm", "compute", "--stdin"];

        let app = create_app();
        let matches = app.get_matches_from_safe(arg_vec).unwrap();

        let submatches = matches.subcommand_matches("compute").unwrap();
        let cmd = compute::<TestingExecutor>(&submatches).unwrap();

        assert_eq!(
            cmd,
            "compute-witness -i target/out -o target/witness -s target/abi.json --abi --stdin"
        )
    }
}
