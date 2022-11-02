use std::fmt::{Display, Formatter};

use subparse::timetypes::TimePoint;

use crate::error::{ApplicationError, ApplicationResult};
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChallengeTimeDifficulty {
    Easy,
    Normal,
    // other difficulties don't have challenge time so not adding them
}

impl ChallengeTimeDifficulty {
    pub fn from_string(difficulty: &str) -> Option<Self> {
        match difficulty.to_lowercase().as_str() {
            "easy" => Some(ChallengeTimeDifficulty::Easy),
            "normal" => Some(ChallengeTimeDifficulty::Normal),
            _ => None,
        }
    }

    pub fn from_integer(difficulty: usize) -> Option<Self> {
        match difficulty {
            0 => Some(ChallengeTimeDifficulty::Easy),
            1 => Some(ChallengeTimeDifficulty::Normal),
            _ => None,
        }
    }
}

impl Display for ChallengeTimeDifficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChallengeTimeDifficulty::Easy => write!(f, "Easy"),
            ChallengeTimeDifficulty::Normal => write!(f, "Normal"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChallengeTime {
    pub difficulty: ChallengeTimeDifficulty,
    pub start: i32,
    pub end: i32,
}

impl ChallengeTime {
    pub fn new(start: i32, end: i32, difficulty: ChallengeTimeDifficulty) -> Self {
        Self {
            start,
            end,
            difficulty,
        }
    }

    pub fn build(
        start_str: String,
        end_str: String,
        difficulty: ChallengeTimeDifficulty,
    ) -> ApplicationResult<Self> {
        let start = parse_challenge_time_timestamp(&start_str);

        if start.is_err() {
            return Err(ApplicationError::InvalidTimestamp(start_str));
        }

        let end = parse_challenge_time_timestamp(&end_str);

        if end.is_err() {
            return Err(ApplicationError::InvalidTimestamp(end_str));
        }

        Ok(Self::new(start.unwrap(), end.unwrap(), difficulty))
    }
}

impl Display for ChallengeTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} ({} difficulty)",
            self.start, self.end, self.difficulty
        )
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

pub fn timestamp_to_millis(ts: TimePoint) -> i32 {
    let minutes = ts.mins_comp();
    let seconds = ts.secs_comp();
    let milliseconds = ts.msecs_comp();

    let minutes_millis = minutes as i32 * 60 * 1000;
    let seconds_millis = seconds as i32 * 1000;

    return minutes_millis + seconds_millis + milliseconds as i32;
}

pub fn parse_challenge_time_timestamp(timestamp: &str) -> ApplicationResult<i32> {
    // format is MM:SS.mmm
    let components = timestamp.split('.').collect::<Vec<&str>>();

    if components.len() != 2 {
        return Err(ApplicationError::InvalidTimestamp(timestamp.to_string()));
    }

    let minutes_and_seconds = components[0].split(':').collect::<Vec<&str>>();
    let milliseconds_str = components[1];

    if minutes_and_seconds.len() != 2 {
        return Err(ApplicationError::InvalidTimestamp(timestamp.to_string()));
    }

    let minutes = minutes_and_seconds[0].parse::<i32>();
    let seconds = minutes_and_seconds[1].parse::<i32>();

    if minutes.is_err() || seconds.is_err() {
        return Err(ApplicationError::InvalidTimestamp(timestamp.to_string()));
    }

    let minutes = minutes.unwrap();
    let seconds = seconds.unwrap();

    if minutes < 0 || minutes > 99 || seconds < 0 || seconds > 59 {
        return Err(ApplicationError::InvalidTimestamp(timestamp.to_string()));
    }

    let milliseconds = milliseconds_str.parse::<i32>();

    if milliseconds.is_err() {
        return Err(ApplicationError::InvalidTimestamp(timestamp.to_string()));
    }

    let milliseconds = milliseconds.unwrap();

    if milliseconds < 0 || milliseconds > 999 {
        return Err(ApplicationError::InvalidTimestamp(timestamp.to_string()));
    }

    let minutes_millis = minutes * 60 * 1000;
    let seconds_millis = seconds * 1000;

    return Ok(minutes_millis + seconds_millis + milliseconds);
}
