use crate::core::constants::DEFAULT_TARGET_DIR;
use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn verify(config: Config) -> Result<(), String> {
    let target = PathBuf::from(DEFAULT_TARGET_DIR);

    let vk_path = target.clone().join("verification.key").into_os_string();
    let proof_path = target.clone().join("proof.json").into_os_string();

    let proof_path = Argument::new("-j", Some(proof_path.to_str().unwrap()));
    let vk_path = Argument::new("-v", Some(vk_path.to_str().unwrap()));

    let backend = Argument::new("-b", Some(config.crypto.backend.as_str()));
    let proving_scheme = Argument::new("-s", Some(config.crypto.proving_scheme.as_str()));
    let curve = Argument::new("-c", Some(config.crypto.elliptic_curve.as_str()));

    let cmd = Command::new(
        "verify",
        vec![proof_path, vk_path, backend, proving_scheme, curve],
    );
    Executor::execute(cmd, false)
}
