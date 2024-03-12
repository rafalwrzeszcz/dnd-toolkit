use std::sync::Arc;
use chrono::NaiveDate;
use dioxus::core::{Element, fc_to_builder, Scope};
use dioxus::core_macro::{component, render};
use dioxus::hooks::{use_future, use_shared_state_provider};
use dioxus_tui::launch;
use dioxus_html as dioxus_elements;
use tracing::info;
use tracing_subscriber::fmt::init;
use rpg_commons_dioxus::ui::AudioPlayButton;
use rpg_commons_native::config::load_from_file;
use rpg_commons_native::rpc::Rpc;
use rpg_commons_native::spotify::Spotify;
use rpg_core::audio::Audio;
use rpg_core::config::AudioConfig;
use rpg_core::context::AppContext;
use rpg_core::game::Game;
use rpg_core::void::Void;

fn app(cx: Scope) -> Element {
    let audio = use_future(cx, (), |_| async move {
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
            AudioConfig::Rpc { url } => Arc::new(Rpc::new(url).await.unwrap()), // TODO
        };
        audio
    });

    use_shared_state_provider(cx, || AppContext { audio: audio.value().unwrap().clone() });

    render!(AudioPlayButton {
        track: "spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into(),
    })
}

fn main() {
    launch(app);
}
