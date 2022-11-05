pub mod simple_logger;

pub trait Logger {
    fn log(&mut self, message: String);

    fn log_lyrics_line(&mut self, line: String);

    fn log_problematic_lyrics_line(&mut self, line: i32, expected: u16, actual: usize);
}
