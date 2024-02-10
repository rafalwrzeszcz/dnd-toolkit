mod audio;
mod config;
mod game;
mod rpc;
mod void;

// TODO: unify wasm and other packages in rpg-commons, figure out way to handle differences with feature toggles

use std::ptr::eq;
use std::sync::Arc;
use chrono::NaiveDate;
use tracing::info;
use tracing_subscriber::fmt::init;
use yew::{function_component, Html, html, Properties, props, Renderer};
use yew::platform::spawn_local;
use crate::audio::Audio;
use crate::config::{AudioConfig, Config, GameMasterConfig, load_from_file};
use crate::game::Game;
use crate::rpc::Rpc;
use crate::void::Void;

#[derive(Properties)]
pub struct AppProps {
    audio: Arc<dyn Audio + Send + Sync + 'static>,
}

impl PartialEq for AppProps {
    fn eq(&self, other: &Self) -> bool {
        eq(self as *const _, other as *const _)
    }
}

#[function_component(App)]
fn app(props: &AppProps) -> Html {
    let audio = props.audio.clone();
    // TODO: use context

    let onclick = move |_| {
        let audio = audio.clone();
        spawn_local(async move {
            audio
                .play("spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into())
                .await
                .unwrap();
        })
    };

    html! {
        <button {onclick}>{ "▶" }</button>
    }
}

fn main() {
    // TODO: init();

    // TODO: let config = load_from_file("config.json".into()).unwrap(); // TODO: config path from param, with default fallback
    let config = Config {
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

    Renderer::<App>::with_props(props! {
        AppProps {
            audio,
        }
    }).render();
}
