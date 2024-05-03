use chrono::NaiveDate;
use glib::{clone, spawn_future_local};
use gtk4::glib::ExitCode;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::traits::{ButtonExt, GtkWindowExt, WidgetExt};
use gtk4::{Application, ApplicationWindow, Button};
use rpg_commons_native::blebox::BleBox;
use rpg_commons_native::config::load_from_file;
use rpg_commons_native::rpc::Rpc;
use rpg_commons_native::spotify::Spotify;
use rpg_core::audio::Audio;
use rpg_core::config::{AudioConfig, LightsConfig};
use rpg_core::game::Game;
use rpg_core::lights::Lights;
use rpg_core::void::Void;
use std::sync::Arc;
use tokio::main as tokio_main;
use tracing::info;
use tracing_subscriber::fmt::init;

#[tokio_main]
async fn main() -> ExitCode {
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

    let lights: Arc<dyn Lights + Send + Sync + 'static> = match config.lights {
        LightsConfig::Void => Arc::new(Void {}),
        LightsConfig::BleBox { host } => Arc::new(BleBox::new(host)), // TODO
        LightsConfig::Rpc { url } => Arc::new(Rpc::new(url).await.unwrap()), // TODO
    };

    let app = Application::builder().application_id("pl.wrzasq.Rpg").build();

    app.connect_activate(move |app| {
        let audio = audio.clone();

        // play button
        let button = Button::builder()
            .label("▶")
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        button.connect_clicked(move |button| {
            spawn_future_local(clone!(@weak button, @strong audio => async move {
                // deactivate the button until the operation is done
                button.set_sensitive(false);

                audio
                    .play("spotify:user:1188797644:playlist:7BkG8gSv69wibGNU2imRMx".into())
                    .await
                    .unwrap();

                // activate the button again
                button.set_sensitive(true);
            }));
        });

        // the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("RPG GTK")
            .child(&button)
            .build();

        window.present();
    });

    app.run()
}
