use serde::Deserialize;
use std::net::SocketAddr;

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
    /// [`gRPC`](crate::rpc::Rpc) implementation.
    ///
    /// # Example
    ///
    /// ```
    /// "audio": {
    ///     "type": "Rpc",
    ///     "url": "http://192.168.0.10:50051/"
    /// }
    /// ```
    Rpc { url: String },
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
    /// [`gRPC`](crate::rpc::Rpc) implementation.
    ///
    /// # Example
    ///
    /// ```
    /// "audio": {
    ///     "type": "Rpc",
    ///     "url": "http://192.168.0.10:50051/"
    /// }
    /// ```
    Rpc { url: String },
}

/// RPC daemon sub-system configuration.
///
/// Specifies setup of RPC listener.
///
/// # Example
///
/// ```
/// "rpc": {
///     "listen": "127.0.0.1:50051"
/// }
/// ```
#[derive(Deserialize)]
pub struct RpcConfig {
    pub listen: SocketAddr,
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
    /// gRPC daemon specification - if omitted, current node will not start RPC listener.
    pub rpc: Option<RpcConfig>,
}
