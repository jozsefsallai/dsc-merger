use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::Arc;

use crate::common::Game;

pub type ApplicationResult<T = ()> = Result<T, ApplicationError>;

#[derive(Debug, Clone)]
pub enum ApplicationError {
    FileNotFound(String),
    UnknownOpcode(i32),
    UnknownOpcodeName(String),
    ArgumentParseError(String, String),
    UnsupportedGame(Game),
    InvalidSubtitleFile,
    WriteFileFailed,
    NoInputFiles,
    InvalidTimestamp(String),
    InvalidDifficultyString(String),
    IOError(Arc<std::io::Error>),
    Utf8ParseError(Arc<std::str::Utf8Error>),
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ApplicationError::FileNotFound(filename) => write!(f, "File not found: {}", filename),
            ApplicationError::UnknownOpcode(opcode) => write!(f, "Unknown opcode: {}", opcode),
            ApplicationError::UnknownOpcodeName(name) => write!(f, "Unknown opcode name: {}", name),
            ApplicationError::ArgumentParseError(opcode, arg) => {
                write!(f, "Invalid command argument for {}: {}", opcode, arg)
            }
            ApplicationError::UnsupportedGame(game) => {
                write!(f, "Unsupported game: {}", game.to_string())
            }
            ApplicationError::InvalidSubtitleFile => write!(f, "Invalid subtitle file"),
            ApplicationError::WriteFileFailed => write!(
                f,
                "Failed to write merged DSC to file (maybe missing permissions?)"
            ),
            ApplicationError::NoInputFiles => write!(f, "You have not specified any input files."),
            ApplicationError::InvalidTimestamp(timestamp) => {
                write!(f, "Invalid timestamp: {}", timestamp)
            }
            ApplicationError::InvalidDifficultyString(difficulty) => {
                write!(f, "Invalid difficulty: {}", difficulty)
            }
            ApplicationError::IOError(error) => write!(f, "IO error: {}", error),
            ApplicationError::Utf8ParseError(error) => write!(f, "Parse error: {}", error),
        }
    }
}

impl Error for ApplicationError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            ApplicationError::IOError(e) => Some(e as &dyn Error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ApplicationError {
    fn from(e: std::io::Error) -> ApplicationError {
        ApplicationError::IOError(Arc::new(e))
    }
}

impl From<std::str::Utf8Error> for ApplicationError {
    fn from(e: std::str::Utf8Error) -> ApplicationError {
        ApplicationError::Utf8ParseError(Arc::new(e))
    }
}
