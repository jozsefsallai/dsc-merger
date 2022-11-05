use crate::logger::Logger;

pub struct GUILogger {
    pub log: String,
    pub lyrics: Vec<String>,
    pub problematic_lyrics_lines: Vec<(i32, u16, usize)>,
}

impl GUILogger {
    pub fn new() -> Self {
        Self {
            log: "Ready.".to_string(),
            lyrics: Vec::new(),
            problematic_lyrics_lines: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.log = "Ready.".to_string();
        self.lyrics.clear();
        self.problematic_lyrics_lines.clear();
    }
}

impl Logger for GUILogger {
    fn log(&mut self, message: String) {
        self.log = message;
    }

    fn log_lyrics_line(&mut self, line: String) {
        self.lyrics.push(line);
    }

    fn log_problematic_lyrics_line(&mut self, line: i32, expected: u16, actual: usize) {
        self.problematic_lyrics_lines.push((line, expected, actual));
    }
}
