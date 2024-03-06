use std::ptr::eq;
use std::sync::Arc;
use chrono::NaiveDate;
use dioxus::core::{Element, fc_to_builder, Scope};
use dioxus::core_macro::render;
use dioxus::hooks::use_shared_state_provider;
use dioxus_html as dioxus_elements;
use dioxus_web::{Config, launch_with_props};
use rpg_commons_dioxus::context::AppContext;
use rpg_commons_dioxus::ui::AudioPlayButton;
use rpg_commons_wasm::config::load_from_file;
use rpg_commons_wasm::rpc::Rpc;
use rpg_core::audio::Audio;
use rpg_core::config::{AudioConfig, Config as RpgConfig, GameMasterConfig};
use rpg_core::game::Game;
use rpg_core::void::Void;
use tracing::info;
use tracing_subscriber::fmt::init;

struct AppProps {
    audio: Arc<dyn Audio + Send + Sync + 'static>,
}

fn app(cx: Scope<AppProps>) -> Element {
    use_shared_state_provider(cx, || AppContext { audio: cx.props.audio.clone() });

    render!(AudioPlayButton {
        track: "spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into(),
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
        AudioConfig::Spotify => panic!("Spotify D-Bus client not available in wasm."), // TODO
    };

    launch_with_props(app, AppProps { audio }, Config::default());
}
