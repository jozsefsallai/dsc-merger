use super::Logger;

pub struct SimpleLogger;

impl SimpleLogger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Logger for SimpleLogger {
    fn log(&mut self, message: String) {
        println!("{}", message);
    }

    fn log_lyrics_line(&mut self, line: String) {
        println!("{}", line);
    }

    fn log_problematic_lyrics_line(&mut self, line: i32, expected: u16, actual: usize) {
        println!(
            "\x1b[33mWarning: Line {} exceeds recommended byte length of {}. Actual length: {}\x1b[39m",
            line, expected, actual
        );
    }
}
