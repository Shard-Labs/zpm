use crate::core::constants::DEFAULT_TARGET_DIR;
use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn verify<E: Executor>(config: Config) -> Result<E::ExecutorResult, String> {
    let target = PathBuf::from(DEFAULT_TARGET_DIR);

    let vk_path = target.clone().join("verification.key").into_os_string();
    let proof_path = target.clone().join("proof.json").into_os_string();

    let proof_path = Argument::new("-j", Some(proof_path.to_str().unwrap()));
    let vk_path = Argument::new("-v", Some(vk_path.to_str().unwrap()));

    let proving_scheme = Argument::new("-s", Some(config.crypto.proving_scheme.as_str()));
    let curve = Argument::new("-c", Some(config.crypto.elliptic_curve.as_str()));

    let cmd = Command::new("verify")
        .args(vec![proof_path, vk_path, proving_scheme, curve])
        .build();

    E::execute(cmd)
}

#[cfg(test)]
mod tests {
    use crate::core::executor::tests::TestingExecutor;
    use crate::core::Config;
    use crate::ops::verify::verify;

    #[test]
    fn verify_command() {
        let config = Config::new("test".to_string());
        let cmd = verify::<TestingExecutor>(config).unwrap();

        assert_eq!(
            cmd,
            "verify -j target/proof.json -v target/verification.key -s g16 -c bn128"
        )
    }
}
