mod audio;
mod config;
mod game;
mod rpc;
mod void;

// TODO: unify wasm and other packages in rpg-commons, figure out way to handle differences with feature toggles

use std::ptr::eq;
use std::sync::Arc;
use chrono::NaiveDate;
use dioxus::core::{Element, Scope};
use dioxus::core_macro::render;
use dioxus_html as dioxus_elements;
use dioxus_web::{Config, launch_with_props};
use tracing::info;
use tracing_subscriber::fmt::init;
use crate::audio::Audio;
use crate::config::{AudioConfig, Config as RpgConfig, GameMasterConfig, load_from_file};
use crate::game::Game;
use crate::rpc::Rpc;
use crate::void::Void;

struct AppProps {
    audio: Arc<dyn Audio + Send + Sync + 'static>,
}

fn app(cx: Scope<AppProps>) -> Element {
    render!(div {
        button {
            onclick: move |_| {
                let audio = cx.props.audio.clone();

                cx.spawn(async move {
                    audio
                        .play("spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into())
                        .await
                        .unwrap();
                });
            },
            "▶"
        }
    })
}


fn main() {
    // TODO: init();

    // TODO: let config = load_from_file("config.json".into()).unwrap(); // TODO: config path from param, with default fallback
    let config = RpgConfig {
        party_name: "Wesoła Kompanija".to_string(),
        game_master: GameMasterConfig {
            name: "Rafał Wrzeszcz".to_string(),
        },
        audio: AudioConfig::Rpc {
            url: "http://127.0.0.1:50051".to_string(),
        },
        rpc: None,
    };

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
        AudioConfig::Rpc { url } => Arc::new(Rpc::new(url)),
    };

    launch_with_props(app, AppProps { audio }, Config::default());
}
