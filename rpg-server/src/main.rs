use chrono::naive::NaiveDate;
use rpg_commons::audio::Audio;
use rpg_commons::config::{load_from_file, AudioConfig};
use rpg_commons::game::Game;
use rpg_commons::rpc::audio_server::AudioServer;
use rpg_commons::rpc::{Listener, Rpc};
use rpg_commons::spotify::Spotify;
use rpg_commons::void::Void;
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

    let (sender, receiver) = channel::<()>();

    // rpc-server
    let rpc = config.rpc.map(|rpc_config| {
        let handler = Listener::new(audio.clone());
        Server::builder()
            .add_service(AudioServer::new(handler))
            .serve_with_shutdown(rpc_config.listen, async { drop(receiver.await) })
    });

    if let Some(server) = rpc {
        server.await.unwrap(); // TODO
        sender.send(()).unwrap(); // TODO - this is needed to keep variable lifetitme to not drop too early
    }
}
