use crate::core::constants::DEFAULT_TARGET_DIR;
use std::path::Path;

pub fn clean() -> Result<(), String> {
    let target = Path::new(DEFAULT_TARGET_DIR);
    let paths = std::fs::read_dir(&target).map_err(|e| {
        format!(
            "Could not read files from target directory ({}): {}",
            target.display(),
            e
        )
    })?;

    for path in paths {
        let file_path = path.unwrap().path();
        debug!("Removing {}", file_path.display());

        std::fs::remove_file(&file_path)
            .map_err(|e| format!("Could not remove {}: {}", file_path.display(), e))?;
    }

    Ok(())
}
