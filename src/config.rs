use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{from_reader, Error as SerdeError};
use std::fmt::Debug;
use std::fs::File;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum ConfigError {
    Io(#[from] IoError),
    Serde(#[from] SerdeError),
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
pub struct SystemConfig {
    /// Audio system configuration.
    pub audio: AudioConfig,
    /// Lights system configuration.
    pub lights: LightsConfig,
}

/// Game configuration structure. It contains all sub-systems configurations.
#[derive(Deserialize)]
pub struct GameConfig {
    /// Name of the game party.
    pub party_name: String,
    /// Game Master definition.
    pub game_master: GameMasterConfig,
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
/// let config: SystemConfig = load_from_file("config.json".into())?;
/// ```
pub fn load_from_file<ConfigType: DeserializeOwned>(path: String) -> Result<ConfigType, ConfigError> {
    let file = File::open(path)?;

    Ok(from_reader(&file)?)
}
