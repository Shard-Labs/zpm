use crate::core::constants::{DEFAULT_SOURCE_DIR, DEFAULT_TARGET_DIR};
use crate::core::Config;
use clap::ArgMatches;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

pub fn create(matches: &ArgMatches) -> Result<(), String> {
    let name = matches.value_of("name").unwrap();
    let path = matches.value_of("path").unwrap();

    debug!("Creating project: {}", name);

    let root = PathBuf::from(path).join(name.clone());
    debug!("Creating project root directory: {}", root.display());

    std::fs::create_dir_all(&root).map_err(|why| {
        format!(
            "Could not create project directory ({}): {}",
            root.display(),
            why
        )
    })?;

    let target_dir_path = root.clone().join(DEFAULT_TARGET_DIR);

    debug!("Creating target directory: {}", target_dir_path.display());
    std::fs::create_dir(&target_dir_path).map_err(|e| {
        format!(
            "Could not create target directory ({}): {}",
            target_dir_path.display(),
            e
        )
    })?;

    let source_dir_path = root.clone().join(DEFAULT_SOURCE_DIR);

    debug!("Creating source directory: {}", source_dir_path.display());
    std::fs::create_dir(&source_dir_path).map_err(|why| {
        format!(
            "Could not create source directory ({}): {}",
            source_dir_path.display(),
            why
        )
    })?;

    let config = Config::new(name.to_string());

    let config_path = root.clone().join("config.zcf");
    let config_file = File::create(&config_path)
        .map_err(|why| format!("Could not create {}: {}", config_path.display(), why))?;

    debug!("Writing: {}", config_path.display());
    let writer = BufWriter::new(config_file);
    config
        .write(writer)
        .map_err(|e| format!("Could not write to {}: {}", config_path.display(), e))?;

    let entry_path = source_dir_path.clone().join(config.general.entry.as_str());

    debug!("Writing: {}", entry_path.display());
    std::fs::write(&entry_path, "def main() -> ():\n    return")
        .map_err(|e| format!("Could not create {}: {}", entry_path.display(), e))?;

    let gitignore = root.clone().join(".gitignore");

    debug!("Writing: {}", gitignore.display());
    std::fs::write(&gitignore, "target")
        .map_err(|e| format!("Could not create {}: {}", entry_path.display(), e))?;

    Ok(())
}
