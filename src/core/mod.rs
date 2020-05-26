use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read, Write};

pub mod constants;
pub mod executor;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub general: General,
    pub crypto: Crypto,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct General {
    pub name: String,
    pub entry: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Crypto {
    pub backend: String,
    pub elliptic_curve: String,
    pub proving_scheme: String,
}

impl Default for Crypto {
    fn default() -> Self {
        Crypto {
            backend: constants::DEFAULT_BACKEND.to_string(),
            elliptic_curve: constants::DEFAULT_ELLIPTIC_CURVE.to_string(),
            proving_scheme: constants::DEFAULT_PROVING_SCHEME.to_string(),
        }
    }
}

impl General {
    pub fn new(name: String) -> Self {
        General {
            name,
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

#[cfg(test)]
mod tests {
    use crate::core::Config;

    #[test]
    fn serialize_config() {
        let config = Config::new("test".to_string());
        let mut tmp = Vec::new();
        config.write(&mut tmp).unwrap();

        assert!(tmp.len() > 0);

        let toml = String::from_utf8(tmp).unwrap();
        let deserialized_config = Config::read(toml.as_bytes()).unwrap();

        assert_eq!(config, deserialized_config);
    }
}
