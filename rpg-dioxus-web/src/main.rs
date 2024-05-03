mod rpc;

use crate::rpc::Rpc;
use chrono::NaiveDate;
use dioxus::core::{fc_to_builder, Element, Scope};
use dioxus::core_macro::render;
use dioxus::hooks::use_shared_state_provider;
use dioxus_html as dioxus_elements;
use dioxus_web::{launch_with_props, Config};
use rpg_commons_dioxus::ui::AudioPlayButton;
use rpg_core::audio::Audio;
use rpg_core::config::{AudioConfig, Config as RpgConfig, GameMasterConfig, LightsConfig};
use rpg_core::context::AppContext;
use rpg_core::game::Game;
use rpg_core::lights::Lights;
use rpg_core::void::Void;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::fmt::init;

fn app(cx: Scope<AppContext>) -> Element {
    use_shared_state_provider(cx, || AppContext {
        audio: cx.props.audio.clone(),
        lights: cx.props.lights.clone(),
    });

    render!(AudioPlayButton {
        track: "spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into(),
    })
}

fn main() {
    // TODO: init();

    // TODO: load from server request
    let config = RpgConfig {
        party_name: "Wesoła Kompanija".to_string(),
        game_master: GameMasterConfig {
            name: "Rafał Wrzeszcz".to_string(),
        },
        audio: AudioConfig::Rpc {
            url: "http://127.0.0.1:50051".to_string(),
        },
        lights: LightsConfig::Rpc {
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

    let lights: Arc<dyn Lights + Send + Sync + 'static> = match config.lights {
        LightsConfig::Void => Arc::new(Void {}),
        LightsConfig::Rpc { url } => Arc::new(Rpc::new(url)), // TODO
        LightsConfig::BleBox { host: _ } => panic!("BleBox REST client not available on mobile."), // TODO
    };

    launch_with_props(app, AppContext { audio, lights }, Config::default());
}
