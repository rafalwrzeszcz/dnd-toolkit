use std::sync::Arc;
use chrono::NaiveDate;
use dioxus::core::{Element, Scope};
use dioxus::core_macro::{component, render};
use dioxus::hooks::use_future;
use dioxus_tui::launch;
use dioxus_html as dioxus_elements;
use dioxus_html::onclick;
use tokio::main as tokio_main;
use tracing::info;
use tracing_subscriber::fmt::init;
use rpg_commons::audio::Audio;
use rpg_commons::config::{AudioConfig, load_from_file};
use rpg_commons::game::Game;
use rpg_commons::rpc::Rpc;
use rpg_commons::spotify::Spotify;
use rpg_commons::void::Void;

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

    render!(div {
        button {
            onclick: move |_| {
                let audio = audio.value().unwrap().clone(); // TODO

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
    launch(app);
}
