/* TODO:

tests (config, audio, lights)

campaign
game session objects
player
character

audio - chromecast (spotify app ID: CC32E753 - https://github.com/azasypkin/rust-caster & https://github.com/aartek/spotify-chromecast-player)
audio - html5 audio player
audio - tabletop audio picker ui
map (grid, generate through AI)
tokens
log
script
scene
monster
npc
initiative list
separate app config from game session config
*/

mod audio;
mod blebox;
mod config;
mod context;
mod game;
mod lights;
mod spotify;
mod void;

use crate::audio::Audio;
use crate::blebox::BleBox;
use crate::config::{load_from_file, AudioConfig, LightsConfig};
use crate::context::AppContext;
use crate::game::Game;
use crate::lights::Lights;
use crate::spotify::Spotify;
use crate::void::Void;
use chrono::naive::NaiveDate;
use clap::{crate_authors, crate_name, crate_version, value_parser};
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result};
use std::sync::Arc;
use tokio::main as tokio_main;
use tracing::info;
use tracing_subscriber::fmt::init;

const CMD_AUDIO_PLAY: &str = "audio:play";
const CMD_LIGHTS_BRIGHTNESS: &str = "lights:brightness";

const ARG_TRACK: &str = "track";
const ARG_LEVEL: &str = "level";

async fn play_audio(args: ArgMatches, ctx: &mut AppContext) -> Result<Option<String>> {
    let track = args.get_one::<String>(ARG_TRACK).unwrap().to_string();
    ctx.audio.play(track).await.unwrap(); // TODO (dedicated error type)

    Ok(None)
}

async fn lights_brightness(args: ArgMatches, ctx: &mut AppContext) -> Result<Option<String>> {
    let level = args.get_one::<i32>(ARG_LEVEL).unwrap().clone();
    ctx.lights.brightness(level).await.unwrap(); // TODO (dedicated error type)

    Ok(None)
}

#[tokio_main]
async fn main() -> Result<()> {
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

    let mut context = AppContext { audio, lights };

    let cmd_audio_play = Command::new(CMD_AUDIO_PLAY)
        // TODO: no sub-command support in repl - existing example is not really handy:
        // https://github.com/arturh85/reedline-repl-rs/blob/main/examples/subcommands.rs
        .arg(Arg::new(ARG_TRACK).required(true))
        .about("Plays audio track.");

    let cmd_lights_brightness = Command::new(CMD_LIGHTS_BRIGHTNESS)
        .arg(Arg::new(ARG_LEVEL).required(true).value_parser(value_parser!(i32)))
        .about("Sets lights brightness.");

    // TODO: try to better unify REPL and CLI args style

    let command = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!(", "))
        .subcommand(cmd_audio_play.clone())
        .subcommand(cmd_lights_brightness.clone())
        .get_matches();

    match command.subcommand() {
        Some((CMD_AUDIO_PLAY, args)) => play_audio(args.clone(), &mut context).await.map(|_| ()),
        Some((CMD_LIGHTS_BRIGHTNESS, args)) => lights_brightness(args.clone(), &mut context).await.map(|_| ()),
        Some(_) => Ok(()), // TODO: print help
        // fallback to REPL interface
        None => {
            Repl::new(context)
                .with_name(crate_name!())
                .with_version(crate_version!())
                .with_description(game.party_name.as_str())
                .with_command_async(cmd_audio_play, |args, context| Box::pin(play_audio(args, context)))
                .with_command_async(cmd_lights_brightness, |args, context| {
                    Box::pin(lights_brightness(args, context))
                })
                .run_async()
                .await
        }
    }
}
