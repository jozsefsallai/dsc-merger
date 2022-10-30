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
