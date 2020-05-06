use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

pub mod constants;
pub mod executor;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub general: General,
    pub crypto: Crypto,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct General {
    pub name: String,
    pub target_dir: String,
    pub source_dir: String,
    pub entry: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crypto {
    pub elliptic_curve: String,
    pub proving_scheme: String,
}

impl Default for Crypto {
    fn default() -> Self {
        Crypto {
            elliptic_curve: constants::DEFAULT_ELLIPTIC_CURVE.to_string(),
            proving_scheme: constants::DEFAULT_PROVING_SCHEME.to_string(),
        }
    }
}

impl General {
    pub fn new(name: String) -> Self {
        General {
            name,
            target_dir: constants::DEFAULT_TARGET_DIR.to_string(),
            source_dir: constants::DEFAULT_SOURCE_DIR.to_string(),
            entry: constants::DEFAULT_ENTRY.to_string(),
        }
    }
}

impl Config {
    pub fn new(name: String) -> Self {
        let general = General::new(name);
        Config {
            general,
            crypto: Crypto::default(),
        }
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        let toml = toml::to_string_pretty(self).unwrap();
        writer.write_all(toml.as_bytes()).unwrap();

        Ok(())
    }
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut toml = String::new();
        reader.read_to_string(&mut toml)?;

        let config: Config = toml::from_str(toml.as_str())?;
        Ok(config)
    }
}
