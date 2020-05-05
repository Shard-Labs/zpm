#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod zpm_core;

use clap::{App, AppSettings, Arg, SubCommand};
use std::env;
use std::io::BufReader;

use crate::zpm_core::commands::compile::CompileCommand;
use crate::zpm_core::commands::init::InitCommand;
use std::fs::File;
use std::path::{Path, PathBuf};
use zpm_core::Config;

fn main() {
    pretty_env_logger::formatted_builder()
        .parse_filters(&env::var("RUST_LOG").unwrap_or("info".to_string()))
        .init();

    cli().unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(1);
    })
}

fn check_path_env(path: &str) -> Result<bool, String> {
    std::env::var("PATH")
        .and_then(|paths| {
            Ok(paths
                .split(":")
                .map(|p| format!("{}/{}", p, path))
                .any(|p| Path::new(&p).exists()))
        })
        .map_err(|e| format!("{}", e))
}

fn read_config(path: &str) -> Result<Config, String> {
    let path = PathBuf::from(path);

    let file = File::open(path.clone())
        .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;

    let reader = BufReader::new(file);
    Config::read(reader).map_err(|e| format!("Error in {}: {}", path.display(), e))
}

fn cli() -> Result<(), String> {
    const CONFIG_DEFAULT_PATH: &str = "config.zcf";

    let matches = App::new("zpm")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("ZoKrates Project Manager")
        .arg(
            Arg::with_name("config-path")
                .global(true)
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("path")
                .required(false)
                .default_value(CONFIG_DEFAULT_PATH),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize new project")
                .display_order(0)
                .arg(
                    Arg::with_name("name")
                        .value_name("name")
                        .help("Sets the project name")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("compile")
                .about("Compile the project")
                .display_order(1),
        )
        .get_matches();

    match check_path_env("zokrates")? {
        false => Err("Could not find zokrates binary in $PATH"),
        _ => Ok(()),
    }?;

    match matches.subcommand() {
        ("init", Some(sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            info!("Creating {}", name);

            let config = Config::new(name.to_string());
            InitCommand::run(config)?
        }
        ("compile", Some(_sub_matches)) => {
            let config: Config = read_config(matches.value_of("config-path").unwrap())
                .map_err(|e| format!("{}", e))?;

            CompileCommand::run(config)?
        }
        _ => unreachable!(),
    }

    Ok(())
}
