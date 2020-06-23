use crate::core::constants::DEFAULT_TARGET_DIR;
use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn generate_proof<E: Executor>(config: Config) -> Result<E::ExecutorResult, String> {
    let target = PathBuf::from(DEFAULT_TARGET_DIR);

    let input = target.clone().join("out").into_os_string();
    let witness = target.clone().join("witness").into_os_string();
    let pk_path = target.clone().join("proving.key").into_os_string();
    let proof_path = target.clone().join("proof.json").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let witness = Argument::new("-w", Some(witness.to_str().unwrap()));
    let pk_path = Argument::new("-p", Some(pk_path.to_str().unwrap()));
    let proof_path = Argument::new("-j", Some(proof_path.to_str().unwrap()));

    let backend = Argument::new("-b", Some(config.crypto.backend.as_str()));
    let proving_scheme = Argument::new("-s", Some(config.crypto.proving_scheme.as_str()));

    let cmd = Command::new("generate-proof")
        .args(vec![
            input,
            witness,
            pk_path,
            proof_path,
            backend,
            proving_scheme,
        ])
        .build();

    E::execute(cmd)
}

#[cfg(test)]
mod tests {
    use crate::core::executor::tests::TestingExecutor;
    use crate::core::Config;
    use crate::ops::generate_proof::generate_proof;

    #[test]
    fn generate_proof_command() {
        let config = Config::new("test".to_string());
        let cmd = generate_proof::<TestingExecutor>(config).unwrap();

        assert_eq!(
            cmd,
            "generate-proof -i target/out -w target/witness -p target/proving.key -j target/proof.json -b bellman -s g16"
        )
    }
}
