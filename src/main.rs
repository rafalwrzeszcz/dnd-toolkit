/* TODO:

do we need arc+mutexes everywhere

cargo make
tests
docs
switch coverage to grcov

campaign

game session objects

player

character

cli interface
web interface
native device display
mobile app

map
audio (document, think about tests)
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
use tokio::sync::Mutex;
use tonic::transport::Server;

use crate::audio::Audio;
use crate::config::{AudioConfig, GameMasterConfig, load_from_file};
use crate::spotify::Spotify;
use crate::rpc::{Listener, Rpc};
use crate::rpc::audio_server::AudioServer;
use crate::void::Void;

struct GameMaster {
    name: String,
}

impl From<GameMasterConfig> for GameMaster {
    fn from(source: GameMasterConfig) -> Self {
        Self {
            name: source.name,
        }
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

async fn play_audio(audio: &dyn Audio) {
    audio.play("spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into()).await;
    // TODO
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

    let audio: Arc<Mutex<dyn Audio + Sync + Send + 'static>> = match config.audio {
        AudioConfig::Void => Arc::new(Mutex::new(Void {})),
        AudioConfig::Spotify => Arc::new(Mutex::new(Spotify::new().unwrap())), // TODO
        AudioConfig::Rpc { url } => Arc::new(Mutex::new(Rpc::new(url).await.unwrap())), // TODO
    };

    let (sender, receiver) = tokio::sync::oneshot::channel::<()>();

    // rpc-server
    let rpc = config.rpc.map(|rpc_config| {
        let handler = Listener::new(audio.clone());
        Server::builder()
            .add_service(AudioServer::new(handler))
            .serve_with_shutdown(rpc_config.listen, async { drop(receiver.await) })
    });

    display_map();

    play_audio(audio.clone().lock().await.deref()).await;

    if let Some(server) = rpc {
        server.await.unwrap(); // TODO
        sender.send(()).unwrap(); // TODO - this is needed to keep variable lifetitme to not drop too early
    }
}
