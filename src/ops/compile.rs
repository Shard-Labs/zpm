use crate::core::constants::{DEFAULT_SOURCE_DIR, DEFAULT_TARGET_DIR};
use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn compile<E: Executor>(config: Config) -> Result<E::ExecutorResult, String> {
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

    let cmd = Command::new("compile")
        .args(vec![input, output, abi_spec, curve])
        .build();

    E::execute(cmd)
}

#[cfg(test)]
mod tests {
    use crate::core::executor::tests::TestingExecutor;
    use crate::core::Config;
    use crate::ops::compile::compile;

    #[test]
    fn compile_command() {
        let config = Config::new("test".to_string());
        let cmd = compile::<TestingExecutor>(config).unwrap();

        assert_eq!(
            cmd,
            "compile -i src/main.zok -o target/out -s target/abi.json -c bn128"
        )
    }
}
