use serde::Deserialize;
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

/// Game Master identification / description.
#[derive(Deserialize)]
pub struct GameMasterConfig {
    /// Display/public name of Game Master.
    pub name: String,
}

/// Audio sub-system configuration.
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum AudioConfig {
    /// No-op, muted implementation [`Void`](crate::void::Void).
    ///
    /// # Example
    ///
    /// ```
    /// "audio": {
    ///     "type": "Void"
    /// }
    /// ```
    Void,
    /// [`Spotify`](crate::spotify::Spotify) D-Bus implementation.
    ///
    /// # Example
    ///
    /// ```
    /// "audio": {
    ///     "type": "Spotify"
    /// }
    /// ```
    Spotify,
}

/// Lights sub-system configuration.
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum LightsConfig {
    /// No-op, muted implementation [`Void`](crate::void::Void).
    ///
    /// # Example
    ///
    /// ```
    /// "audio": {
    ///     "type": "Void"
    /// }
    /// ```
    Void,
    /// [`BleBox`](crate::blebox::BleBox) REST API client.
    ///
    /// # Example
    ///
    /// ```
    /// "audio": {
    ///     "type": "BleBox",
    ///     "host": "http://192.168.0.11/"
    /// }
    /// ```
    BleBox { host: String },
}

/// Overall system configuration structure. It contains all sub-systems configurations.
#[derive(Deserialize)]
pub struct Config {
    /// Name of the game party.
    pub party_name: String,
    /// Game Master definition.
    pub game_master: GameMasterConfig,
    /// Audio system configuration.
    pub audio: AudioConfig,
    /// Lights system configuration.
    pub lights: LightsConfig,
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
