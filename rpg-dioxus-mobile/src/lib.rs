mod config;
mod rpc;

use crate::rpc::Rpc;
use anyhow::Result;
use chrono::NaiveDate;
use dioxus::core::{fc_to_builder, Element, Scope};
use dioxus::core_macro::render;
use dioxus::hooks::use_shared_state_provider;
use dioxus_desktop::{launch_with_props, Config};
use dioxus_html as dioxus_elements;
use log::info;
use rpg_commons_dioxus::ui::AudioPlayButton;
use rpg_core::audio::Audio;
use rpg_core::config::{AudioConfig, Config as RpgConfig, GameMasterConfig, LightsConfig};
use rpg_core::context::AppContext;
use rpg_core::game::Game;
use rpg_core::lights::Lights;
use rpg_core::void::Void;
use std::sync::Arc;
use tokio::main as tokio_main;
#[cfg(target_os = "android")]
use wry::android_binding;

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace)
            .with_tag("rpg-dioxus-mobile"),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
    env_logger::init();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn _start_app() {
    stop_unwind(|| main().unwrap());
}

#[no_mangle]
#[inline(never)]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub extern "C" fn start_app() {
    #[cfg(target_os = "android")]
    android_binding!(pl_wrzasq, rpg_dioxus_mobile, _start_app);
    #[cfg(target_os = "ios")]
    _start_app()
}

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
pub async fn main() -> Result<()> {
    init_logging();

    // TODO: configuration handling
    let config = RpgConfig {
        party_name: "Wesoła Kompanija".to_string(),
        game_master: GameMasterConfig {
            name: "Rafał Wrzeszcz".to_string(),
        },
        audio: AudioConfig::Rpc {
            url: "http://10.0.2.2:50051".to_string(),
        },
        lights: LightsConfig::Rpc {
            url: "http://10.0.2.2:50051".to_string(),
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
        AudioConfig::Rpc { url } => Arc::new(Rpc::new(url).await.unwrap()), // TODO
        AudioConfig::Spotify => panic!("Spotify D-Bus client not available on mobile."), // TODO
    };

    let lights: Arc<dyn Lights + Send + Sync + 'static> = match config.lights {
        LightsConfig::Void => Arc::new(Void {}),
        LightsConfig::Rpc { url } => Arc::new(Rpc::new(url).await.unwrap()), // TODO
        LightsConfig::BleBox { host: _ } => panic!("BleBox REST client not available on mobile."), // TODO
    };

    launch_with_props(
        app,
        AppContext { audio, lights },
        Config::default().with_custom_index(r#"<!DOCTYPE html>
        <html>
          <head>
            <title>Dioxus app</title>
            <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no" />
          </head>
          <body>
            <div id="main"></div>
          </body>
        </html>
       "#.into()),
    );

    Ok(())
}
