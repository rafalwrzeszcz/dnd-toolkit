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

bring back #1dd1504-#2a2aaa8
*/

mod audio;
mod blebox;
mod config;
mod context;
mod game;
mod lights;
mod spotify;
mod void;

use crate::audio::{Audio, AudioError};
use crate::blebox::BleBox;
use crate::config::{load_from_file, AudioConfig, GameConfig, LightsConfig, SystemConfig};
use crate::context::AppContext;
use crate::game::Game;
use crate::lights::{Lights, LightsError};
use crate::spotify::{Spotify, SpotifyError};
use crate::void::Void;
use chrono::naive::NaiveDate;
use clap::{crate_authors, crate_name, crate_version, value_parser};
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Error as ReplError, Repl};
use std::fmt::Debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::main as tokio_main;
use tracing::info;
use tracing_subscriber::fmt::init;

const CMD_AUDIO_PLAY: &str = "audio:play";
const CMD_LIGHTS_BRIGHTNESS: &str = "lights:brightness";

const ARG_TRACK: &str = "track";
const ARG_LEVEL: &str = "level";

async fn play_audio(args: ArgMatches, ctx: &mut AppContext) -> Result<Option<String>, AppError> {
    let track = args
        .get_one::<String>(ARG_TRACK)
        .ok_or(ReplError::MissingRequiredArgument(
            CMD_AUDIO_PLAY.to_string(),
            ARG_TRACK.to_string(),
        ))?
        .to_string();
    ctx.audio.play(track).await?;

    Ok(None)
}

async fn lights_brightness(args: ArgMatches, ctx: &mut AppContext) -> Result<Option<String>, AppError> {
    let level = args
        .get_one::<i32>(ARG_LEVEL)
        .ok_or(ReplError::MissingRequiredArgument(
            CMD_LIGHTS_BRIGHTNESS.to_string(),
            ARG_LEVEL.to_string(),
        ))?
        .clone();
    ctx.lights.brightness(level).await?;

    Ok(None)
}

#[derive(Error, Debug)]
#[error(transparent)]
enum AppError {
    Repl(#[from] ReplError),
    Audio(#[from] AudioError),
    Lights(#[from] LightsError),
    Spotify(#[from] SpotifyError),
}

#[tokio_main]
async fn main() -> Result<(), AppError> {
    init();

    let system_config: SystemConfig = load_from_file("config.json".into()).unwrap(); // TODO: config path from param, with default fallback
    let game_config: GameConfig = load_from_file("game.json".into()).unwrap(); // TODO: config path from param, with default fallback

    let game = Game {
        party_name: game_config.party_name,
        date: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(), // TODO
        game_master: game_config.game_master.into(),
    };

    info!("{}", game.party_name);
    info!("{}", game.date);
    info!("{}", game.game_master.name);

    let audio: Arc<dyn Audio + Send + Sync + 'static> = match system_config.audio {
        AudioConfig::Void => Arc::new(Void {}),
        AudioConfig::Spotify => Arc::new(Spotify::new()?),
    };

    let lights: Arc<dyn Lights + Send + Sync + 'static> = match system_config.lights {
        LightsConfig::Void => Arc::new(Void {}),
        LightsConfig::BleBox { host } => Arc::new(BleBox::new(host)),
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

    Ok(match command.subcommand() {
        Some((CMD_AUDIO_PLAY, args)) => play_audio(args.clone(), &mut context).await.map(|_| ())?,
        Some((CMD_LIGHTS_BRIGHTNESS, args)) => lights_brightness(args.clone(), &mut context).await.map(|_| ())?,
        Some(_) => (), // TODO: print help
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
                .await?
        }
    })
}
