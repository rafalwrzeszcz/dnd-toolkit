use chrono::naive::NaiveDate;
use rpg_commons_native::blebox::BleBox;
use rpg_commons_native::config::load_from_file;
use rpg_commons_native::rpc::audio_server::AudioServer;
use rpg_commons_native::rpc::Rpc;
use rpg_commons_native::spotify::Spotify;
use rpg_core::audio::Audio;
use rpg_core::config::{AudioConfig, LightsConfig};
use rpg_core::context::AppContext;
use rpg_core::game::Game;
use rpg_core::lights::Lights;
use rpg_core::void::Void;
use std::sync::Arc;
use tokio::main as tokio_main;
use tokio::sync::oneshot::channel;
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::fmt::init;

#[tokio_main]
async fn main() {
    init();

    let config = load_from_file("config.json".into()).unwrap(); // TODO: config path from param, with default fallback

    let game = Game {
        party_name: config.party_name,
        date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(), // TODO
        game_master: config.game_master.into(),
    };

    info!("{}", game.party_name);
    info!("{}", game.date);
    info!("{}", game.game_master.name);

    let audio: Arc<dyn Audio + Send + Sync + 'static> = match config.audio {
        AudioConfig::Void => Arc::new(Void {}),
        AudioConfig::Spotify => Arc::new(Spotify::new().unwrap()), // TODO
        AudioConfig::Rpc { url } => Arc::new(Rpc::new(url).await.unwrap()), // TODO
    };

    let lights: Arc<dyn Lights + Send + Sync + 'static> = match config.lights {
        LightsConfig::Void => Arc::new(Void {}),
        LightsConfig::BleBox { host } => Arc::new(BleBox::new(host)), // TODO
        LightsConfig::Rpc { url } => Arc::new(Rpc::new(url).await.unwrap()), // TODO
    };

    let (sender, receiver) = channel::<()>();

    // rpc-server
    let rpc = config.rpc.map(|rpc_config| {
        let handler = AppContext {
            audio: audio.clone(),
            lights: lights.clone(),
        };
        Server::builder()
            .accept_http1(true)
            .add_service(AudioServer::new(handler))
            .serve_with_shutdown(rpc_config.listen, async { drop(receiver.await) })
    });

    if let Some(server) = rpc {
        server.await.unwrap(); // TODO
        sender.send(()).unwrap(); // TODO - this is needed to keep variable lifetitme to not drop too early
    }
}
