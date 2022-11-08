#![allow(uncommon_codepoints)]

use std::env;

use clap::Parser;
use common::{ChallengeTime, ChallengeTimeDifficulty, Game};
use error::ApplicationResult;

#[cfg(feature = "gui")]
use gui::GUI;

#[cfg(not(feature = "gui"))]
use interactive::InteractiveTUI;
use logger::simple_logger::SimpleLogger;

use crate::application::Application;

mod application;
mod common;
mod dsc;
mod error;
#[cfg(feature = "gui")]
mod gui;
#[cfg(not(feature = "gui"))]
mod interactive;
mod logger;
mod merger;
mod opcodes;
mod subtitle;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    // Input paths
    #[arg(long, short)]
    input: Vec<String>,

    #[arg(long, short)]
    plaintext_input: Vec<String>,

    #[arg(long, short)]
    subtitle_input: Vec<String>,

    // Output path
    #[arg(short, long, default_value = "output.dsc")]
    output: String,

    // Game name
    #[arg(long, short, default_value = "FT")]
    game: String,

    // Lyrics-related arguments
    #[arg(long, default_value = "0")]
    pv_id: u16,

    #[arg(long)]
    english_lyrics: bool,

    #[arg(long, default_value = "75")]
    max_lyric_length: u16,

    // Challenge time arguments
    #[arg(long, required = false)]
    ct_start: Option<String>,

    #[arg(long, required = false)]
    ct_end: Option<String>,

    #[arg(long, required = false)]
    difficulty: Option<String>,

    // Debug arguments
    #[arg(long)]
    dump: bool,

    #[arg(long, short)]
    verbose: bool,
}

fn get_challenge_time_object(args: &Arguments) -> ApplicationResult<Option<ChallengeTime>> {
    if args.ct_start.is_some() && args.ct_end.is_some() && args.difficulty.is_some() {
        let fallback = "".to_string();

        let start_str = args.ct_start.as_ref().unwrap_or(&fallback);
        let end_str = args.ct_end.as_ref().unwrap_or(&fallback);
        let difficulty_str = args.difficulty.as_ref().unwrap_or(&fallback);
        let difficulty = ChallengeTimeDifficulty::from_string(&difficulty_str);

        match difficulty {
            Some(difficulty) => {
                let challenge_time =
                    ChallengeTime::build(start_str.to_string(), end_str.to_string(), difficulty);

                match challenge_time {
                    Ok(challenge_time) => Ok(Some(challenge_time)),
                    Err(e) => Err(e),
                }
            }
            None => Err(error::ApplicationError::InvalidDifficultyString(
                difficulty_str.to_string(),
            )),
        }
    } else {
        Ok(None)
    }
}

fn main() {
    let argc = env::args().len();

    if argc == 1 {
        // User probably double-clicked the exe
        #[cfg(feature = "gui")]
        GUI::new().run();

        #[cfg(not(feature = "gui"))]
        InteractiveTUI::start();

        return;
    }

    let args = Arguments::parse();

    let game = match args.game.to_lowercase().as_str() {
        "f" => Game::F,
        "f2" | "f2nd" | "f 2nd" => Game::F2nd,
        "x" => Game::X,
        "ft" | "futuretone" | "future tone" => Game::FutureTone,
        "arcade" | "aft" => Game::Arcade,
        _ => {
            println!("Invalid game: {}", args.game);
            return;
        }
    };

    let challenge_time = get_challenge_time_object(&args);

    if challenge_time.is_err() {
        println!("{}", challenge_time.unwrap_err());
        return;
    }

    let mut logger = SimpleLogger::new();

    let mut application = Application::new(
        args.input,
        args.plaintext_input,
        args.subtitle_input,
        args.output,
        game,
        args.pv_id,
        args.english_lyrics,
        args.max_lyric_length,
        args.dump,
        args.verbose,
        challenge_time.unwrap(),
        &mut logger,
    );

    match application.run() {
        Ok(_) => {
            println!("Done!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
