/* TODO:

switch coverage to grcov
tests (config, rpc, audio)
docs (general project, audio, config)
tracing

campaign

game session objects

player

character

cli interface
web interface
native device display
mobile app

map
tokens
log
script
scene
monster
npc
initiative list
*/

mod audio;
mod config;
mod rpc;
mod spotify;
mod void;

use chrono::naive::NaiveDate;
use env_logger::Builder;
use log::info;
use std::convert::From;
use std::ops::Deref;
use std::sync::Arc;
use tokio::main as tokio_main;
use tokio::sync::oneshot::channel;
use tonic::transport::Server;

use crate::audio::{Audio, AudioError};
use crate::config::{load_from_file, AudioConfig, GameMasterConfig};
use crate::rpc::audio_server::AudioServer;
use crate::rpc::{Listener, Rpc};
use crate::spotify::Spotify;
use crate::void::Void;

struct GameMaster {
    name: String,
}

impl From<GameMasterConfig> for GameMaster {
    fn from(source: GameMasterConfig) -> Self {
        Self { name: source.name }
    }
}

struct Game {
    party_name: String,
    date: NaiveDate,
    game_master: GameMaster,
}

fn display_map() {
    // TODO
}

async fn play_audio(audio: &dyn Audio) -> Result<(), AudioError> {
    audio
        .play("spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into())
        .await
}

#[tokio_main]
async fn main() {
    Builder::from_default_env().init();

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

    display_map();

    play_audio(audio.deref()).await.unwrap();

    if let Some(server) = rpc {
        server.await.unwrap(); // TODO
        sender.send(()).unwrap(); // TODO - this is needed to keep variable lifetitme to not drop too early
    }
}
