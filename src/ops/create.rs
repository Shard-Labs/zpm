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

    let project_root_path = PathBuf::from(path).join(name.clone());
    debug!(
        "Creating project root directory: {}",
        project_root_path.display()
    );

    std::fs::create_dir_all(&project_root_path).map_err(|why| {
        format!(
            "Could not create project directory ({}): {}",
            project_root_path.display(),
            why
        )
    })?;

    let result = {
        let target_dir_path = project_root_path.clone().join(DEFAULT_TARGET_DIR);
        debug!("Creating target directory: {}", target_dir_path.display());

        std::fs::create_dir(&target_dir_path).map_err(|e| {
            format!(
                "Could not create target directory ({}): {}",
                target_dir_path.display(),
                e
            )
        })?;

        let source_dir_path = project_root_path.clone().join(DEFAULT_SOURCE_DIR);
        debug!("Creating source directory: {}", source_dir_path.display());

        std::fs::create_dir(&source_dir_path).map_err(|why| {
            format!(
                "Could not create source directory ({}): {}",
                source_dir_path.display(),
                why
            )
        })?;

        let config = Config::new(name.to_string());
        let config_path = project_root_path.clone().join("config.zcf");
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

        let gitignore = project_root_path.clone().join(".gitignore");
        debug!("Writing: {}", gitignore.display());

        std::fs::write(&gitignore, "target")
            .map_err(|e| format!("Could not create {}: {}", entry_path.display(), e))?;

        Ok(())
    };

    // cleanup if something went wrong while creating the project
    if result.is_err() {
        std::fs::remove_dir_all(&project_root_path).expect("Could not cleanup project");
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::create_app;
    use crate::ops::create::create;
    use tempdir::TempDir;

    #[test]
    fn create_command() {
        let tmp_dir = TempDir::new(".tmp").unwrap();
        let arg_vec = vec!["zpm", "create", "test", tmp_dir.path().to_str().unwrap()];

        let app = create_app();
        let matches = app.get_matches_from_safe(arg_vec).unwrap();

        let submatches = matches.subcommand_matches("create").unwrap();
        assert!(create(submatches).is_ok());
    }
}
