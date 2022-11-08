use std::{fs::File, io::Read};

use subparse::{SrtFile, SsaFile, SubtitleEntry, SubtitleFileInterface};

use crate::common::{get_lyric_command, get_time_command, timestamp_to_millis};
use crate::error::{ApplicationError, ApplicationResult};
use crate::logger::Logger;
use crate::opcodes::Command;

pub enum SubtitleKind {
    SRT,
    ASS,
}

impl SubtitleKind {
    pub fn from_extension(extension: &str) -> Option<Self> {
        let extension = extension.to_lowercase();

        match extension.as_str() {
            "srt" => Some(SubtitleKind::SRT),
            "ass" | "ssa" => Some(SubtitleKind::ASS),
            _ => None,
        }
    }
}

pub struct SubtitleFile<'a> {
    entries: Vec<SubtitleEntry>,
    logger: &'a mut dyn Logger,
}

impl<'a> SubtitleFile<'a> {
    pub fn load_srt(file: &mut File, logger: &'a mut dyn Logger) -> ApplicationResult<Self> {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let file_contents = std::str::from_utf8(&buffer)?;

        let srt = SrtFile::parse(file_contents);

        match srt {
            Ok(srt) => {
                let entries = srt.get_subtitle_entries().unwrap_or(Vec::new());
                Ok(Self { entries, logger })
            }
            Err(_) => return Err(ApplicationError::InvalidSubtitleFile),
        }
    }

    pub fn load_ass(file: &mut File, logger: &'a mut dyn Logger) -> ApplicationResult<Self> {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let file_contents = std::str::from_utf8(&buffer)?;

        let ass = SsaFile::parse(file_contents);

        match ass {
            Ok(ass) => {
                let entries = ass.get_subtitle_entries().unwrap_or(Vec::new());
                Ok(Self { entries, logger })
            }
            Err(_) => return Err(ApplicationError::InvalidSubtitleFile),
        }
    }

    pub fn create_lyric_commands(
        &mut self,
        pv_id: u16,
        is_english: bool,
        max_line_length: u16,
    ) -> ApplicationResult<Vec<Command>> {
        let mut command_buffer = Vec::new();

        let mut idx = 1;

        let pv_id = format!("pv_{:0>3}", pv_id);
        let key = match is_english {
            true => "lyric_en",
            false => "lyric",
        };

        let mut problematic_lines = Vec::new();

        let mut last_end_time_ms = 0;

        for subtitle in &self.entries {
            let line = &subtitle.line;

            if line.is_none() {
                continue;
            }

            let line = line.clone().unwrap_or("".to_string());
            let clean_line = line.trim();

            let start_time_ms = timestamp_to_millis(subtitle.timespan.start);
            let end_time_ms = timestamp_to_millis(subtitle.timespan.end);

            if start_time_ms == last_end_time_ms {
                // No need to reset the lyrics since the previous line's end is
                // at the same time as the current line's start. So just remove
                // the last LYRIC and TIME commands.
                command_buffer.remove(command_buffer.len() - 1);
                command_buffer.remove(command_buffer.len() - 1);
            }

            last_end_time_ms = end_time_ms;

            let start_time_command = get_time_command(start_time_ms * 100);
            command_buffer.push(start_time_command);

            let lyric_command = get_lyric_command(idx, -1);
            command_buffer.push(lyric_command);

            let end_time_command = get_time_command(end_time_ms * 100);
            command_buffer.push(end_time_command);

            let lyric_reset_command = get_lyric_command(0, -1);
            command_buffer.push(lyric_reset_command);

            let formatted_id = format!("{:0>3}", idx);
            let line = format!("{}.{}.{}={}", pv_id, key, formatted_id, clean_line);

            if line.len() > max_line_length as usize {
                problematic_lines.push((idx, line.len()));
            }

            self.logger.log_lyrics_line(line.clone());

            idx += 1;
        }

        for line in problematic_lines {
            self.logger
                .log_problematic_lyrics_line(line.0, max_line_length, line.1);
        }

        Ok(command_buffer)
    }
}
