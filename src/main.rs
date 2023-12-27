/* TODO:

results
async

campaign

game session objects

player

character

local and over network

cli interface
web interface
native device display
mobile app

map
audio (local, over network, spotify - qdbus org.mpris.MediaPlayer2.spotify /org/mpris/MediaPlayer2 org.mpris.MediaPlayer2.Player.OpenUri spotify:track:1WNPappMd13lY5o9POZ4gU)
tokens
log
script
scene
monster
npc
initiative list

config file
*/

mod audio;
mod rpc;
mod spotify;
mod void;

use chrono::naive::NaiveDate;
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
    let game = Game {
        party_name: "Wesoła Kompanija".into(),
        date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(), // TODO
        game_master: GameMaster {
            name: "Rafał Wrzeszcz".into(),
        },
    };

    println!("{}", game.party_name);
    println!("{}", game.date);
    println!("{}", game.game_master.name);

    //let audio = Spotify::new();
    //let audio = Void {};

    display_map();

    // rpc-client
    let audio = Rpc::new().await;
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
