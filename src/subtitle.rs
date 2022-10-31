use std::{fs::File, io::Read};

use srtlib::Subtitles;

use crate::common::{get_lyric_command, get_time_command, timestamp_to_millis};
use crate::error::{ApplicationError, ApplicationResult};
use crate::opcodes::Command;

pub struct SubtitleFile {
    subtitles: Subtitles,
}

impl SubtitleFile {
    pub fn load(file: &mut File) -> ApplicationResult<Self> {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let subtitles =
            Subtitles::parse_from_str(std::str::from_utf8(&buffer).unwrap().to_string());

        match subtitles {
            Ok(subtitles) => Ok(SubtitleFile { subtitles }),
            Err(_) => Err(ApplicationError::InvalidSubtitleFile),
        }
    }

    pub fn create_lyric_commands(
        self,
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

        for subtitle in self.subtitles.to_vec() {
            let start_time_ms = timestamp_to_millis(subtitle.start_time);
            let end_time_ms = timestamp_to_millis(subtitle.end_time);

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

            let clean_text = subtitle.text.trim();

            let formatted_id = format!("{:0>3}", idx);
            let line = format!("{}.{}.{}={}", pv_id, key, formatted_id, clean_text);

            println!("{}", line);

            if line.len() > max_line_length as usize {
                problematic_lines.push((idx, line.len()));
            }

            idx += 1;
        }

        for line in problematic_lines {
            println!("\x1b[33mWarning: Line {} exceeds recommended byte length of {}. Actual length: {}\x1b[39m", line.0, max_line_length, line.1);
        }

        Ok(command_buffer)
    }
}
