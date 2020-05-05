use crate::zpm_core::Config;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

pub struct InitCommand {}

impl InitCommand {
    pub fn run(config: Config) -> Result<(), String> {
        let root = PathBuf::from(env::current_dir().unwrap()).join(config.general.name.as_str());

        std::fs::create_dir(root.clone())
            .map_err(|why| format!("Could not create directory {} : {}", root.display(), why))?;

        let config_path = root.clone().join("config.zcf");
        let config_file = File::create(config_path.clone())
            .map_err(|why| format!("Could not create {}: {}", config_path.display(), why))?;

        let writer = BufWriter::new(config_file);
        config
            .write(writer)
            .map_err(|e| format!("Could not write {}: {}", config_path.display(), e))?;

        let source_dir_path = root.clone().join(config.general.source_dir.as_str());
        let target_dir_path = root.clone().join(config.general.target_dir.as_str());

        std::fs::create_dir(&source_dir_path).map_err(|why| {
            format!(
                "Could not create source directory ({}): {}",
                source_dir_path.display(),
                why
            )
        })?;

        std::fs::create_dir(&target_dir_path).map_err(|e| {
            format!(
                "Could not create target directory ({}): {}",
                target_dir_path.display(),
                e
            )
        })?;

        let entry_path = source_dir_path.clone().join(config.general.entry.as_str());

        std::fs::write(&entry_path, "def main() -> ():\n    return")
            .map_err(|e| format!("Could not create {}: {}", entry_path.display(), e))?;

        Ok(())
    }
}
