#![allow(uncommon_codepoints)]

use std::fs::File;

use clap::Parser;
use common::Game;
use dsc::DSCVM;
use error::{ApplicationError, ApplicationResult};
use merger::DSCMerger;

mod common;
mod dsc;
mod error;
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

    #[arg(short, long)]
    output: String,

    #[arg(long, short)]
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

fn handle_file(game: Game, filename: &str) -> ApplicationResult<DSCVM> {
    let file = std::fs::File::open(filename);

    match file {
        Ok(mut file) => {
            let dsc_vm = DSCVM::load(game, &mut file)?;

            Ok(dsc_vm)
        }
        Err(_) => Err(ApplicationError::FileNotFound(filename.to_owned())),
    }
}

fn handle_plaintext_file(game: Game, filename: &str) -> ApplicationResult<DSCVM> {
    let file = std::fs::File::open(filename);

    match file {
        Ok(mut file) => {
            let dsc_vm = DSCVM::load_plaintext(game, &mut file)?;

            Ok(dsc_vm)
        }
        Err(_) => Err(ApplicationError::FileNotFound(filename.to_owned())),
    }
}

fn handle_subtitle_file(
    filename: &str,
    pv_id: u16,
    is_english: bool,
    max_line_length: u16,
) -> ApplicationResult<DSCVM> {
    let file = std::fs::File::open(filename);

    match file {
        Ok(mut file) => {
            let dsc_vm = DSCVM::load_subtitle(&mut file, pv_id, is_english, max_line_length)?;

            Ok(dsc_vm)
        }
        Err(_) => Err(ApplicationError::FileNotFound(filename.to_owned())),
    }
}

fn main() {
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

    if args.verbose {
        println!(
            "Merging charts for target game: Project Diva {}.",
            game.to_string()
        );
    }

    let mut merger = DSCMerger::new();

    for filename in args.input {
        if args.verbose {
            println!("Loading DSC file: \"{}\"...", filename);
        }

        let dsc_vm = handle_file(game, &filename);

        match dsc_vm {
            Ok(dsc_vm) => merger.add_dsc(dsc_vm),
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }

    for filename in args.plaintext_input {
        if args.verbose {
            println!("Loading plaintext/dumped DSC file: \"{}\"...", filename);
        }

        let dsc_vm = handle_plaintext_file(game, &filename);

        match dsc_vm {
            Ok(dsc_vm) => merger.add_dsc(dsc_vm),
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }

    for filename in args.subtitle_input {
        if args.verbose {
            println!("Loading subtitle file: \"{}\"", filename);
        }

        let dsc_vm = handle_subtitle_file(
            &filename,
            args.pv_id,
            args.english_lyrics,
            args.max_lyric_length,
        );

        match dsc_vm {
            Ok(dsc_vm) => merger.add_dsc(dsc_vm),
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }

    if args.verbose {
        println!("Merging DSC commands...")
    }

    let new_dsc = merger.to_dsc();

    if args.dump {
        println!("{}", new_dsc.dump());
    }

    if args.verbose {
        println!("Writing merged DSC to file: \"{}\"...", args.output);
    }

    let output_file = File::create(args.output);

    if let Ok(mut output_file) = output_file {
        new_dsc.write(game, &mut output_file).unwrap();
    } else {
        println!("Failed to write merged DSC to file (maybe missing permissions?)");
    }

    println!("Done!");
}
