use crate::core::constants::DEFAULT_TARGET_DIR;
use crate::core::executor::{Argument, Command, Executor};
use crate::core::Config;
use std::path::PathBuf;

pub fn export_verifier(config: Config) -> Result<(), String> {
    let target = PathBuf::from(DEFAULT_TARGET_DIR);

    let input = target.clone().join("verification.key").into_os_string();
    let output = target.clone().join("verifier.sol").into_os_string();

    let input = Argument::new("-i", Some(input.to_str().unwrap()));
    let output = Argument::new("-o", Some(output.to_str().unwrap()));

    let backend = Argument::new("-b", Some(config.crypto.backend.as_str()));
    let proving_scheme = Argument::new("-s", Some(config.crypto.proving_scheme.as_str()));
    let curve = Argument::new("-c", Some(config.crypto.elliptic_curve.as_str()));

    let cmd = Command::new(
        "export-verifier",
        vec![input, output, backend, proving_scheme, curve],
    );

    Executor::execute(cmd, false)
}
