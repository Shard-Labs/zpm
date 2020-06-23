use crate::core::constants::DEFAULT_TARGET_DIR;
use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn export_verifier<E: Executor>(config: Config) -> Result<E::ExecutorResult, String> {
    let target = PathBuf::from(DEFAULT_TARGET_DIR);

    let input = target.clone().join("verification.key").into_os_string();
    let output = target.clone().join("verifier.sol").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let output = Argument::new("-o", Some(output.to_str().unwrap()));

    let proving_scheme = Argument::new("-s", Some(config.crypto.proving_scheme.as_str()));
    let curve = Argument::new("-c", Some(config.crypto.elliptic_curve.as_str()));

    let cmd = Command::new("export-verifier")
        .args(vec![input, output, proving_scheme, curve])
        .build();

    E::execute(cmd)
}

#[cfg(test)]
mod tests {
    use crate::core::executor::tests::TestingExecutor;
    use crate::core::Config;
    use crate::ops::export_verifier::export_verifier;

    #[test]
    fn export_verifier_command() {
        let config = Config::new("test".to_string());
        let cmd = export_verifier::<TestingExecutor>(config).unwrap();

        assert_eq!(
            cmd,
            "export-verifier -i target/verification.key -o target/verifier.sol -s g16 -c bn128"
        )
    }
}
