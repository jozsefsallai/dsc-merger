use srtlib::Timestamp;

use crate::opcodes::{Command, Opcode, OpcodeMeta};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Game {
    F,
    F2nd,
    X,
    FutureTone,
    Arcade,
}

impl Game {
    pub fn to_string(&self) -> String {
        match self {
            Game::F => "F",
            Game::F2nd => "F2nd",
            Game::X => "X",
            Game::FutureTone => "Future Tone",
            Game::Arcade => "Arcade",
        }
        .to_string()
    }
}

pub fn get_time_command(time: i32) -> Command {
    let id = 1;
    let opcode = Opcode::TIME;
    let param_count: usize = 1;

    let meta = OpcodeMeta::new(id, opcode, param_count);
    return Command::new(meta, vec![time]);
}

pub fn get_lyric_command(idx: i32, mode: i32) -> Command {
    let id = 24;
    let opcode = Opcode::LYRIC;
    let param_count: usize = 2;

    let meta = OpcodeMeta::new(id, opcode, param_count);
    return Command::new(meta, vec![idx, mode]);
}

pub fn timestamp_to_millis(ts: Timestamp) -> i32 {
    let (hours, minutes, seconds, milliseconds) = ts.get();

    let hours_millis = hours as i32 * 60 * 60 * 1000;
    let minutes_millis = minutes as i32 * 60 * 1000;
    let seconds_millis = seconds as i32 * 1000;

    return hours_millis + minutes_millis + seconds_millis + milliseconds as i32;
}
