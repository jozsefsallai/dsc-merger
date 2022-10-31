use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::common::Game;

pub type ApplicationResult<T = ()> = Result<T, ApplicationError>;

#[derive(Debug, Clone)]
pub enum ApplicationError {
    FileNotFound(String),
    UnknownOpcode(i32),
    UnknownOpcodeName(String),
    UnsupportedGame(Game),
    InvalidSubtitleFile,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ApplicationError::FileNotFound(filename) => write!(f, "File not found: {}", filename),
            ApplicationError::UnknownOpcode(opcode) => write!(f, "Unknown opcode: {}", opcode),
            ApplicationError::UnknownOpcodeName(name) => write!(f, "Unknown opcode name: {}", name),
            ApplicationError::UnsupportedGame(game) => {
                write!(f, "Unsupported game: {}", game.to_string())
            }
            ApplicationError::InvalidSubtitleFile => write!(f, "Invalid subtitle file"),
        }
    }
}
