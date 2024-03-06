use rpg_core::config::Config;
use serde_json::{from_reader, Error as SerdeError};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    Io(#[from] IoError),
    Serde(#[from] SerdeError),
}

impl Display for ConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{self:?}")
    }
}

/// Loads configuration from specified JSON configuration file.
///
/// # Arguments
///
/// * `path` - Configuration file location.
///
/// # Example
///
/// ```
/// let config = load_from_file("config.json".into())?;
/// ```
pub fn load_from_file(path: String) -> Result<Config, ConfigError> {
    let file = File::open(path)?;

    Ok(from_reader(&file)?)
}
