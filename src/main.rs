#![allow(uncommon_codepoints)]

use std::env;

use clap::Parser;
use common::Game;
use interactive::InteractiveTUI;

use crate::application::Application;

mod application;
mod common;
mod dsc;
mod error;
mod interactive;
mod merger;
mod opcodes;
mod subtitle;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(long, short)]
    input: Vec<String>,

    #[arg(long, short)]
    plaintext_input: Vec<String>,

    #[arg(long, short)]
    subtitle_input: Vec<String>,

    #[arg(short, long, default_value = "output.dsc")]
    output: String,

    #[arg(long, short, default_value = "FT")]
    game: String,

    #[arg(long, default_value = "0")]
    pv_id: u16,

    #[arg(long)]
    english_lyrics: bool,

    #[arg(long, default_value = "75")]
    max_lyric_length: u16,

    #[arg(long)]
    dump: bool,

    #[arg(long, short)]
    verbose: bool,
}

fn main() {
    let argc = env::args().len();

    if argc == 1 {
        // User probably double-clicked the exe
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

    let application = Application::new(
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
