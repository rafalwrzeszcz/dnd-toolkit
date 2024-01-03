use serde::Deserialize;
use serde_json::{from_reader, Error as SerdeError};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::Error as IoError;
use std::net::SocketAddr;
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

#[derive(Deserialize)]
pub struct GameMasterConfig {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum AudioConfig {
    Void,
    Spotify,
    Rpc { url: String },
}

#[derive(Deserialize)]
pub struct RpcConfig {
    pub listen: SocketAddr,
}

#[derive(Deserialize)]
pub struct Config {
    pub party_name: String,
    pub game_master: GameMasterConfig,
    pub audio: AudioConfig,
    pub rpc: Option<RpcConfig>,
}

pub fn load_from_file(path: String) -> Result<Config, ConfigError> {
    let file = File::open(path)?;

    Ok(from_reader(&file)?)
}
