use crate::core::constants::DEFAULT_TARGET_DIR;
use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn setup<E: Executor>(config: Config) -> Result<E::ExecutorResult, String> {
    let target = PathBuf::from(DEFAULT_TARGET_DIR);
    let input = target.clone().join("out").into_os_string();

    let vk_path = target.clone().join("verification.key").into_os_string();

    let pk_path = target.clone().join("proving.key").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let vk_path = Argument::new("-v", Some(vk_path.to_str().unwrap()));
    let pk_path = Argument::new("-p", Some(pk_path.to_str().unwrap()));

    let proving_scheme = Argument::new("-s", Some(config.crypto.proving_scheme.as_str()));
    let cmd = Command::new("setup")
        .args(vec![input, vk_path, pk_path, proving_scheme])
        .build();

    E::execute(cmd)
}

#[cfg(test)]
mod tests {
    use crate::core::executor::tests::TestingExecutor;
    use crate::core::Config;
    use crate::ops::setup::setup;

    #[test]
    fn setup_command() {
        let config = Config::new("test".to_string());
        let cmd = setup::<TestingExecutor>(config).unwrap();

        assert_eq!(
            cmd,
            "setup -i target/out -v target/verification.key -p target/proving.key -s g16"
        )
    }
}
