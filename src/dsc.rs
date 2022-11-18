use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use crate::error::{ApplicationError, ApplicationResult};
use crate::opcodes::Command;
use crate::subtitle::{SubtitleFile, SubtitleKind};
use crate::{common::Game, logger::Logger};

pub struct DSCVM {
    pub command_buffer: Vec<Command>,
    pub remove_targets: bool,
}

impl DSCVM {
    pub fn new(remove_targets: bool) -> Self {
        Self {
            command_buffer: Vec::new(),
            remove_targets,
        }
    }

    pub fn add_command(&mut self, command: Command) {
        self.command_buffer.push(command);
    }

    pub fn load(game: Game, file: &mut File, remove_targets: bool) -> ApplicationResult<Self> {
        let mut command_buffer = Vec::new();

        let skip = match game {
            Game::F | Game::FutureTone => 4,
            Game::F2nd | Game::X => 72,
            _ => 0,
        };

        file.seek(SeekFrom::Start(skip as u64))?;

        loop {
            let opcode = file.read_i32::<LE>()?;

            if opcode == 0 || (opcode == 1128681285 && (game == Game::F2nd || game == Game::X)) {
                break;
            }

            let opcode_meta = Command::get_opcode_meta(game, opcode)?;

            let mut args = Vec::new();

            for _ in 0..opcode_meta.param_count {
                let arg = file.read_i32::<LE>()?;
                args.push(arg);
            }

            command_buffer.push(Command::new(opcode_meta, args));
        }

        let end = Command::get_opcode_meta(game, 0).unwrap();

        command_buffer.push(Command::new(end, vec![]));

        Ok(Self {
            command_buffer,
            remove_targets,
        })
    }

    pub fn load_plaintext(
        game: Game,
        file: &mut File,
        remove_targets: bool,
    ) -> ApplicationResult<Self> {
        let mut command_buffer = Vec::new();

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let normalized_line = line
                .unwrap_or("".to_string())
                .trim()
                .replace(")", "")
                .replace(";", "")
                .replace(" ", "");

            if normalized_line.len() == 0 || normalized_line.starts_with("#") {
                continue;
            }

            let components = normalized_line.split("(").collect::<Vec<&str>>();

            if components.len() != 2 {
                continue;
            }

            let opcode_name = components[0];

            let opcode_meta = Command::get_opcode_meta_from_name(game, opcode_name.to_owned());

            match opcode_meta {
                Ok(opcode_meta) => {
                    let opcode_args_list = components[1].split(",").collect::<Vec<&str>>();

                    let mut args = Vec::new();

                    for raw_arg in opcode_args_list.iter() {
                        match raw_arg.parse::<i32>() {
                            Ok(arg) => {
                                args.push(arg);
                            }
                            Err(_) => {
                                return Err(ApplicationError::ArgumentParseError(
                                    opcode_name.to_string(),
                                    raw_arg.to_string(),
                                ));
                            }
                        }
                    }

                    command_buffer.push(Command::new(opcode_meta, args));
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(Self {
            command_buffer,
            remove_targets,
        })
    }

    pub fn load_subtitle<'a>(
        file: &mut File,
        kind: SubtitleKind,
        pv_id: u16,
        is_english: bool,
        max_line_length: u16,
        logger: &'a mut dyn Logger,
    ) -> ApplicationResult<Self> {
        let subtitle_file = match kind {
            SubtitleKind::SRT => SubtitleFile::load_srt(file, logger),
            SubtitleKind::ASS => SubtitleFile::load_ass(file, logger),
        };

        match subtitle_file {
            Ok(mut subtitle_file) => {
                match subtitle_file.create_lyric_commands(pv_id, is_english, max_line_length) {
                    Ok(command_buffer) => Ok(Self {
                        command_buffer,
                        remove_targets: false,
                    }),
                    Err(err) => Err(err),
                }
            }
            Err(_) => Err(ApplicationError::InvalidSubtitleFile),
        }
    }

    pub fn dump(&self) -> String {
        let mut output = String::new();

        for command in &self.command_buffer {
            output.push_str(&command.to_string());
            output.push_str("\n");
        }

        output
    }

    pub fn write(&self, game: Game, file: &mut File) -> ApplicationResult {
        match game {
            Game::F => {
                file.write_i32::<LE>(302121504)?;
            }
            Game::FutureTone => {
                file.write_i32::<LE>(335874337)?;
            }
            Game::F2nd | Game::X => {
                file.write_i32::<LE>(1129535056)?;

                for _ in 0..18 {
                    file.write_i32::<LE>(0)?;
                }
            }
            _ => {}
        };

        for command in &self.command_buffer {
            file.write_i32::<LE>(command.meta.id)?;

            for arg in &command.args {
                file.write_i32::<LE>(*arg)?;
            }
        }

        Ok(())
    }
}
