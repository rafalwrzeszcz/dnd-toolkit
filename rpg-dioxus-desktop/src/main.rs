use std::sync::Arc;
use chrono::NaiveDate;
use dioxus::core::{Element, Scope};
use dioxus::core_macro::{component, render};
use dioxus_desktop::{Config, launch_with_props};
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
        AudioConfig::Rpc { url } => Arc::new(Rpc::new(url).await.unwrap()), // TODO
    };

    launch_with_props(app, AppProps { audio }, Config::default());
}
