use std::ptr::eq;
use std::sync::Arc;
use chrono::NaiveDate;
use rpg_commons_wasm::config::load_from_file;
use rpg_commons_wasm::rpc::Rpc;
use rpg_core::audio::Audio;
use rpg_core::config::{AudioConfig, Config, GameMasterConfig};
use rpg_core::game::Game;
use rpg_core::void::Void;
use tracing::info;
use tracing_subscriber::fmt::init;
use yew::{function_component, Html, html, Properties, props, Renderer};
use yew::platform::spawn_local;

#[derive(Properties)]
pub struct AppProps {
    audio: Arc<dyn Audio + Send + Sync + 'static>,
}

impl PartialEq for AppProps {
    fn eq(&self, other: &Self) -> bool {
        eq(self.audio.as_ref() as *const _, other.audio.as_ref() as *const _)
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
        AudioConfig::Spotify => panic!("Spotify D-Bus client not available in wasm."), // TODO
    };

    Renderer::<App>::with_props(props! {
        AppProps {
            audio,
        }
    }).render();
}
