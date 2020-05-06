use crate::core::Config;
use std::path::PathBuf;

pub fn clean(config: Config) -> Result<(), String> {
    let target = PathBuf::from(config.general.target_dir);
    let paths = std::fs::read_dir(&target).map_err(|e| {
        format!(
            "Could not read files from target directory ({}): {}",
            target.display(),
            e
        )
    })?;

    for path in paths {
        let file_path = path.unwrap().path();
        info!("Removing {}", file_path.display());

        std::fs::remove_file(&file_path)
            .map_err(|e| format!("Could not remove {}: {}", file_path.display(), e))?;
    }

    Ok(())
}