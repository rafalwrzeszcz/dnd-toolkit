/* TODO:

config file

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
mod rpc;
mod spotify;
mod void;

use chrono::naive::NaiveDate;
use env_logger::Builder;
use log::info;
use tokio::main as tokio_main;

use crate::audio::Audio;
use crate::spotify::Spotify;
use crate::rpc::Rpc;
use crate::void::Void;

struct GameMaster {
    name: String,
}

struct Game {
    party_name: String,
    date: NaiveDate,
    game_master: GameMaster,
}

fn display_map() {
    // TODO
}

async fn play_audio<T: Audio>(audio: &T) {
    audio.play("spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into()).await;
    // TODO
}

#[tokio_main]
async fn main() {
    Builder::from_default_env().init();

    let game = Game {
        party_name: "Wesoła Kompanija".into(),
        date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(), // TODO
        game_master: GameMaster {
            name: "Rafał Wrzeszcz".into(),
        },
    };

    info!("{}", game.party_name);
    info!("{}", game.date);
    info!("{}", game.game_master.name);

    //let audio = Spotify::new().unwrap();
    //let audio = Void {};

    display_map();

    // rpc-client
    let audio = Rpc::new("http://127.0.0.1:50051".to_string()).await.unwrap();
    play_audio(&audio).await;
    
    // rpc-server
    // let (sender, receiver) = tokio::sync::oneshot::channel::<()>();
    // let handler = crate::rpc::Listener::new(Spotify::new());
    // let server = tonic::transport::Server::builder()
    //     .add_service(crate::rpc::audio_server::AudioServer::new(handler))
    //     .serve_with_shutdown("127.0.0.1:50051".parse().unwrap(), async { drop(receiver.await) });
    // //sender.send(()).unwrap();
    // server.await;
}
