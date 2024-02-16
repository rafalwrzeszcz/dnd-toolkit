use chrono::naive::NaiveDate;
use rpg_commons::audio::Audio;
use rpg_commons::config::{load_from_file, AudioConfig};
use rpg_commons::game::Game;
use rpg_commons::rpc::Rpc;
use rpg_commons::spotify::Spotify;
use rpg_commons::void::Void;
use std::sync::Arc;
use clap::{crate_authors, crate_name, crate_version};
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result};
use tokio::main as tokio_main;
use tracing::info;
use tracing_subscriber::fmt::init;

const CMD_AUDIO_PLAY: &str = "audio:play";

const ARG_TRACK: &str = "track";

struct ReplContext {
    audio: Arc<dyn Audio + Send + Sync + 'static>,
}

async fn play_audio(args: ArgMatches, ctx: &mut ReplContext) -> Result<Option<String>> {
    let track = args.get_one::<String>(ARG_TRACK).unwrap().to_string();
    ctx.audio
        .play(track)
        .await
        .unwrap(); // TODO

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
        AudioConfig::Rpc { url } => Arc::new(Rpc::new(url).await.unwrap()), // TODO
    };

    let mut context = ReplContext {
        audio,
    };

    let cmd_audio_play = Command::new(CMD_AUDIO_PLAY)
        // TODO: no sub-command support in repl - existing example is not really handy:
        // https://github.com/arturh85/reedline-repl-rs/blob/main/examples/subcommands.rs
        .arg(Arg::new(ARG_TRACK).required(true))
        .about("Plays audio track.");

    // TODO: try to better unify REPL and CLI args style

    let command = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!(", "))
        .subcommand(cmd_audio_play.clone())
        .get_matches();

    match command.subcommand() {
        Some((CMD_AUDIO_PLAY, &ref args)) => play_audio(args.clone(), &mut context).await.map(|_| ()),
        Some(_) => Ok(()), // TODO: print help
        // fallback to REPL interface
        None => Repl::new(context)
            .with_name(crate_name!())
            .with_version(crate_version!())
            .with_description(game.party_name.as_str())
            .with_command_async(
                cmd_audio_play,
                |args, context| Box::pin(play_audio(args, context)),
            )
            .run_async()
            .await
    }
}
