mod ui;

use crate::ui::AudioPlayButton;
use chrono::NaiveDate;
use dioxus::core::{fc_to_builder, Element, Scope};
use dioxus::core_macro::{component, render};
use dioxus::hooks::use_shared_state_provider;
use dioxus_desktop::{launch_with_props, Config};
use dioxus_html as dioxus_elements;
use rpg_core::audio::Audio;
use rpg_core::blebox::BleBox;
use rpg_core::config::load_from_file;
use rpg_core::config::{AudioConfig, LightsConfig};
use rpg_core::context::AppContext;
use rpg_core::game::Game;
use rpg_core::lights::Lights;
use rpg_core::spotify::Spotify;
use rpg_core::void::Void;
use std::sync::Arc;
use tokio::main as tokio_main;
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
    };

    let lights: Arc<dyn Lights + Send + Sync + 'static> = match config.lights {
        LightsConfig::Void => Arc::new(Void {}),
        LightsConfig::BleBox { host } => Arc::new(BleBox::new(host)), // TODO
    };

    launch_with_props(app, AppContext { audio, lights }, Config::default());
}
