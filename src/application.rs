use std::fs::File;

use crate::common::Game;
use crate::dsc::DSCVM;
use crate::error::{ApplicationError, ApplicationResult};
use crate::merger::DSCMerger;

pub struct Application {
    dsc_inputs: Vec<String>,
    plaintext_inputs: Vec<String>,
    subtitle_inputs: Vec<String>,
    output: String,
    game: Game,
    pv_id: u16,
    english_lyrics: bool,
    max_lyric_length: u16,
    dump: bool,
    verbose: bool,
}

impl Application {
    pub fn new(
        dsc_inputs: Vec<String>,
        plaintext_inputs: Vec<String>,
        subtitle_inputs: Vec<String>,
        output: String,
        game: Game,
        pv_id: u16,
        english_lyrics: bool,
        max_lyric_length: u16,
        dump: bool,
        verbose: bool,
    ) -> Self {
        Self {
            dsc_inputs,
            plaintext_inputs,
            subtitle_inputs,
            output,
            game,
            pv_id,
            english_lyrics,
            max_lyric_length,
            dump,
            verbose,
        }
    }

    fn handle_file(&self, filename: &str) -> ApplicationResult<DSCVM> {
        let file = File::open(filename);

        match file {
            Ok(mut file) => {
                let dsc_vm = DSCVM::load(self.game, &mut file)?;

                Ok(dsc_vm)
            }
            Err(_) => Err(ApplicationError::FileNotFound(filename.to_owned())),
        }
    }

    fn handle_plaintext_file(&self, filename: &str) -> ApplicationResult<DSCVM> {
        let file = std::fs::File::open(filename);

        match file {
            Ok(mut file) => {
                let dsc_vm = DSCVM::load_plaintext(self.game, &mut file)?;

                Ok(dsc_vm)
            }
            Err(_) => Err(ApplicationError::FileNotFound(filename.to_owned())),
        }
    }

    fn handle_subtitle_file(&self, filename: &str) -> ApplicationResult<DSCVM> {
        let file = std::fs::File::open(filename);

        match file {
            Ok(mut file) => {
                let dsc_vm = DSCVM::load_subtitle(
                    &mut file,
                    self.pv_id,
                    self.english_lyrics,
                    self.max_lyric_length,
                )?;

                Ok(dsc_vm)
            }
            Err(_) => Err(ApplicationError::FileNotFound(filename.to_owned())),
        }
    }

    pub fn run(&self) -> ApplicationResult {
        if self.verbose {
            println!(
                "Merging charts for target game: Project Diva {}.",
                self.game.to_string()
            );
        }

        let mut merger = DSCMerger::new();

        for filename in &self.dsc_inputs {
            if self.verbose {
                println!("Loading DSC file: \"{}\"...", filename);
            }

            let dsc_vm = self.handle_file(&filename);

            match dsc_vm {
                Ok(dsc_vm) => merger.add_dsc(dsc_vm),
                Err(e) => {
                    return Err(e);
                }
            }
        }

        for filename in &self.plaintext_inputs {
            if self.verbose {
                println!("Loading plaintext/dumped DSC file: \"{}\"...", filename);
            }

            let dsc_vm = self.handle_plaintext_file(&filename);

            match dsc_vm {
                Ok(dsc_vm) => merger.add_dsc(dsc_vm),
                Err(e) => {
                    return Err(e);
                }
            }
        }

        for filename in &self.subtitle_inputs {
            if self.verbose {
                println!("Loading subtitle file: \"{}\"", filename);
            }

            let dsc_vm = self.handle_subtitle_file(&filename);

            match dsc_vm {
                Ok(dsc_vm) => merger.add_dsc(dsc_vm),
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if self.verbose {
            println!("Merging DSC commands...")
        }

        let new_dsc = merger.to_dsc();

        if self.dump {
            println!("{}", new_dsc.dump());
        }

        if self.verbose {
            println!("Writing merged DSC to file: \"{}\"...", self.output);
        }

        let output_file = File::create(&self.output);

        if let Ok(mut output_file) = output_file {
            new_dsc.write(self.game, &mut output_file).unwrap();

            Ok(())
        } else {
            Err(ApplicationError::WriteFileFailed)
        }
    }
}
