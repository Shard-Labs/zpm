#[macro_use]
extern crate log;
extern crate env_logger;

use clap::{App, AppSettings, Arg, SubCommand};
use std::env;
use std::io::BufReader;

mod core;
mod ops;

use crate::ops::clean::clean;
use crate::ops::compile::compile;
use crate::ops::compute::compute;
use crate::ops::create::create;
use crate::ops::export_verifier::export_verifier;
use crate::ops::generate_proof::generate_proof;
use crate::ops::setup::setup;

use crate::core::constants::CONFIG_DEFAULT_PATH;
use crate::core::Config;
use log::LevelFilter;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    let level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env_logger::builder()
        .format(|f, record| {
            writeln!(
                f,
                "{: <5} zpm: {}",
                f.default_styled_level(record.level()),
                record.args()
            )
        })
        .filter_level(LevelFilter::from_str(&level).unwrap())
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
        .map_err(|e| format!("Error in $PATH: {}", e))
}

fn read_config(path: &str) -> Result<Config, String> {
    let path = PathBuf::from(path);

    let file = File::open(path.clone())
        .map_err(|why| format!("Could not open {}: {}", path.display(), why))?;

    let reader = BufReader::new(file);
    Config::read(reader).map_err(|e| format!("Error in {}: {}", path.display(), e))
}

fn cli() -> Result<(), String> {
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
            SubCommand::with_name("create")
                .about("Creates new project")
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
                .about("Compiles the project")
                .display_order(1),
        )
        .subcommand(
            SubCommand::with_name("setup")
                .about("Performs a trusted setup for a given constraint system")
                .display_order(2),
        )
        .subcommand(
            SubCommand::with_name("compute")
                .about("Calculates a witness for a given constraint system")
                .display_order(3)
                .arg(
                    Arg::with_name("arguments")
                        .short("a")
                        .long("arguments")
                        .help("Arguments for the compiled program")
                        .takes_value(true)
                        .multiple(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("stdin")
                        .long("stdin")
                        .help("Read arguments from stdin")
                        .conflicts_with("arguments")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("export-verifier")
                .about("Exports a verifier as Solidity smart contract")
                .display_order(4),
        )
        .subcommand(
            SubCommand::with_name("generate-proof")
                .about("Calculates a proof for a given constraint system and witness")
                .display_order(5),
        )
        .subcommand(
            SubCommand::with_name("clean")
                .about("Cleans target directory")
                .display_order(6),
        )
        .get_matches();

    match check_path_env("zokrates")? {
        false => Err("Could not find zokrates binary in $PATH"),
        _ => Ok(()),
    }?;

    match matches.subcommand() {
        ("create", Some(sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            let config = Config::new(name.to_string());

            create(config)?
        }
        ("compile", _) => {
            let config: Config = read_config(matches.value_of("config-path").unwrap())
                .map_err(|e| format!("{}", e))?;

            compile(config)?
        }
        ("setup", _) => {
            let config: Config = read_config(matches.value_of("config-path").unwrap())
                .map_err(|e| format!("{}", e))?;

            setup(config)?
        }
        ("compute", Some(sub_matches)) => compute(sub_matches)?,
        ("export-verifier", _) => {
            let config: Config = read_config(matches.value_of("config-path").unwrap())
                .map_err(|e| format!("{}", e))?;

            export_verifier(config)?
        }
        ("generate-proof", _) => {
            let config: Config = read_config(matches.value_of("config-path").unwrap())
                .map_err(|e| format!("{}", e))?;

            generate_proof(config)?
        }
        ("clean", _) => clean()?,
        _ => unreachable!(),
    }

    Ok(())
}
