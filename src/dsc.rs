use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use crate::common::Game;
use crate::error::ApplicationResult;
use crate::opcodes::Command;

pub struct DSCVM {
    pub command_buffer: Vec<Command>,
}

impl DSCVM {
    pub fn new() -> Self {
        Self {
            command_buffer: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: Command) {
        self.command_buffer.push(command);
    }

    pub fn load(game: Game, file: &mut File) -> ApplicationResult<Self> {
        let mut command_buffer = Vec::new();

        let skip = match game {
            Game::F | Game::FutureTone => 4,
            Game::F2nd | Game::X => 72,
            _ => 0,
        };

        file.seek(SeekFrom::Start(skip as u64)).unwrap();

        loop {
            let opcode = file.read_i32::<LE>().unwrap();

            if opcode == 0 || (opcode == 1128681285 && (game == Game::F2nd || game == Game::X)) {
                break;
            }

            let opcode_meta = Command::get_opcode_meta(game, opcode)?;

            let mut args = Vec::new();

            for _ in 0..opcode_meta.param_count {
                let arg = file.read_i32::<LE>().unwrap();
                args.push(arg);
            }

            command_buffer.push(Command::new(opcode_meta, args));
        }

        let end = Command::get_opcode_meta(game, 0).unwrap();

        command_buffer.push(Command::new(end, vec![]));

        Ok(Self { command_buffer })
    }

    pub fn load_plaintext(game: Game, file: &mut File) -> ApplicationResult<Self> {
        let mut command_buffer = Vec::new();

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let normalized_line = line
                .unwrap()
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

            let mut opcode_args_list = components[1].split(",").collect::<Vec<&str>>();
            let opcode_args = opcode_args_list
                .iter_mut()
                .map(|x| x.parse::<i32>().unwrap());

            let opcode_meta = Command::get_opcode_meta_from_name(game, opcode_name.to_owned());

            match opcode_meta {
                Ok(opcode_meta) => {
                    let mut args = Vec::new();

                    for arg in opcode_args {
                        args.push(arg);
                    }

                    command_buffer.push(Command::new(opcode_meta, args));
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(Self { command_buffer })
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
                file.write_i32::<LE>(302121504).unwrap();
            }
            Game::FutureTone => {
                file.write_i32::<LE>(335874337).unwrap();
            }
            Game::F2nd | Game::X => {
                file.write_i32::<LE>(1129535056).unwrap();

                for _ in 0..18 {
                    file.write_i32::<LE>(0).unwrap();
                }
            }
            _ => {}
        };

        for command in &self.command_buffer {
            file.write_i32::<LE>(command.meta.id).unwrap();

            for arg in &command.args {
                file.write_i32::<LE>(*arg).unwrap();
            }
        }

        Ok(())
    }
}
