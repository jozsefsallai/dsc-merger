use std::fs::File;

use crate::common::{ChallengeTime, Game};
use crate::dsc::DSCVM;
use crate::error::{ApplicationError, ApplicationResult};
use crate::logger::Logger;
use crate::merger::DSCMerger;
use crate::subtitle::SubtitleKind;

pub struct Application<'a> {
    dsc_inputs: Vec<String>,
    plaintext_inputs: Vec<String>,
    subtitle_inputs: Vec<String>,
    remove_targets_inputs: Vec<String>,
    output: String,
    game: Game,
    pv_id: u16,
    english_lyrics: bool,
    max_lyric_length: u16,
    dump: bool,
    verbose: bool,
    challenge_time: Option<ChallengeTime>,

    logger: &'a mut dyn Logger,
}

impl<'a> Application<'a> {
    pub fn new(
        dsc_inputs: Vec<String>,
        plaintext_inputs: Vec<String>,
        subtitle_inputs: Vec<String>,
        remove_targets_inputs: Vec<String>,
        output: String,
        game: Game,
        pv_id: u16,
        english_lyrics: bool,
        max_lyric_length: u16,
        dump: bool,
        verbose: bool,
        challenge_time: Option<ChallengeTime>,
        logger: &'a mut dyn Logger,
    ) -> Self {
        Self {
            dsc_inputs,
            plaintext_inputs,
            subtitle_inputs,
            remove_targets_inputs,
            output,
            game,
            pv_id,
            english_lyrics,
            max_lyric_length,
            dump,
            verbose,
            challenge_time,
            logger,
        }
    }

    fn handle_file(&self, filename: &str) -> ApplicationResult<DSCVM> {
        let file = File::open(filename);
        let remove_targets = self.remove_targets_inputs.contains(&filename.to_string());

        match file {
            Ok(mut file) => {
                let dsc_vm = DSCVM::load(self.game, &mut file, remove_targets)?;

                Ok(dsc_vm)
            }
            Err(_) => Err(ApplicationError::FileNotFound(filename.to_owned())),
        }
    }

    fn handle_plaintext_file(&self, filename: &str) -> ApplicationResult<DSCVM> {
        let file = std::fs::File::open(filename);
        let remove_targets = self.remove_targets_inputs.contains(&filename.to_string());

        match file {
            Ok(mut file) => {
                let dsc_vm = DSCVM::load_plaintext(self.game, &mut file, remove_targets)?;

                Ok(dsc_vm)
            }
            Err(_) => Err(ApplicationError::FileNotFound(filename.to_owned())),
        }
    }

    fn handle_subtitle_file(&mut self, filename: &str) -> ApplicationResult<DSCVM> {
        let extension = filename.split('.').last().unwrap_or("srt");
        let kind = SubtitleKind::from_extension(extension);

        if kind.is_none() {
            return Err(ApplicationError::InvalidSubtitleFile);
        }

        let file = std::fs::File::open(filename);

        match file {
            Ok(mut file) => {
                let dsc_vm = DSCVM::load_subtitle(
                    &mut file,
                    kind.unwrap(),
                    self.pv_id,
                    self.english_lyrics,
                    self.max_lyric_length,
                    self.logger,
                )?;

                Ok(dsc_vm)
            }
            Err(_) => Err(ApplicationError::FileNotFound(filename.to_owned())),
        }
    }

    pub fn run(&mut self) -> ApplicationResult {
        if self.verbose {
            self.logger.log(format!(
                "Merging charts for target game: Project Diva {}.",
                self.game.to_string()
            ));
        }

        if self.dsc_inputs.len() == 0
            && self.plaintext_inputs.len() == 0
            && self.subtitle_inputs.len() == 0
        {
            return Err(ApplicationError::NoInputFiles);
        }

        let mut merger = DSCMerger::new();

        for filename in &self.dsc_inputs {
            if self.verbose {
                self.logger
                    .log(format!("Loading DSC file: \"{}\"...", filename));
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
                self.logger.log(format!(
                    "Loading plaintext/dumped DSC file: \"{}\"...",
                    filename
                ));
            }

            let dsc_vm = self.handle_plaintext_file(&filename);

            match dsc_vm {
                Ok(dsc_vm) => merger.add_dsc(dsc_vm),
                Err(e) => {
                    return Err(e);
                }
            }
        }

        for filename in self.subtitle_inputs.clone() {
            if self.verbose {
                self.logger
                    .log(format!("Loading subtitle file: \"{}\"...", filename));
            }

            let dsc_vm = self.handle_subtitle_file(&filename);

            match dsc_vm {
                Ok(dsc_vm) => merger.add_dsc(dsc_vm),
                Err(e) => {
                    return Err(e);
                }
            }
        }

        match self.challenge_time {
            Some(challenge_time) => {
                if self.verbose {
                    self.logger
                        .log(format!("Adding challenge time: {}", challenge_time));
                    merger.add_challenge_time(challenge_time);
                }
            }
            None => {}
        }

        if self.verbose {
            self.logger.log("Merging DSC commands...".to_string())
        }

        let new_dsc = merger.to_dsc();

        if self.dump {
            println!("{}", new_dsc.dump());
        }

        if self.verbose {
            self.logger.log(format!(
                "Writing merged DSC to file: \"{}\"...",
                self.output
            ));
        }

        let output_file = File::create(&self.output);

        if let Ok(mut output_file) = output_file {
            new_dsc.write(self.game, &mut output_file)
        } else {
            Err(ApplicationError::WriteFileFailed)
        }
    }
}
