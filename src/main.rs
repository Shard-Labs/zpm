#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env;
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    pretty_env_logger::formatted_builder()
        .parse_filters(&env::var("RUST_LOG").unwrap_or("info".to_string()))
        .init();

    cli().unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(1);
    })
}

fn cli() -> Result<(), String> {
    const CONFIG_DEFAULT_PATH: &str = "config.zcf";

    let matches = App::new("zpm")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("ZoKrates Project Manager")
        .subcommand(SubCommand::with_name("init")
            .about("Initialize new project")
            .display_order(0)
            .arg(Arg::with_name("name")
                .value_name("name")
                .help("Sets the project name")
                .takes_value(true)
                .required(true)
            )
        ).subcommand(SubCommand::with_name("compile")
            .about("Compile the project")
            .display_order(1)
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("file")
                .help("ZoKrates configuration file")
                .takes_value(true)
                .required(false)
                .default_value(CONFIG_DEFAULT_PATH)
            )
        ).get_matches();

    match matches.subcommand() {
        ("init", Some(sub_matches)) => {
            info!("Creating {}", sub_matches.value_of("name").unwrap())
            // TODO: create project structure with default values
        },
        ("compile", Some(_sub_matches)) => {
            // TODO: parse configuration file, run compilation
        },
        _ => unreachable!()
    }

    Ok(())
}