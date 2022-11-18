use imgui::FontId;

use crate::{
    application::Application,
    common::{ChallengeTime, ChallengeTimeDifficulty, Game, GAME_MAP},
    error::ApplicationResult,
};

use super::gui_logger::GUILogger;

pub struct GUIState {
    pub logger: GUILogger,

    pub font_id: Option<FontId>,
    pub font_id_jp: Option<FontId>,

    pub dsc_inputs: Vec<String>,
    pub plaintext_inputs: Vec<String>,
    pub subtitle_inputs: Vec<String>,
    pub remove_targets_map: Vec<(String, bool)>,

    pub output: String,
    game: Game,

    pub selected_game_index: usize,

    pub selected_dsc_index: usize,
    pub selected_plaintext_index: usize,
    pub selected_subtitle_index: usize,

    pub has_challenge_time: bool,
    pub selected_difficulty_index: usize,
    selected_difficulty: Option<ChallengeTimeDifficulty>,
    pub pv_id: i32,
    pub challenge_time_start: String,
    pub challenge_time_end: String,
    pub english_lyrics: bool,
    pub max_lyric_length: i32,

    pub is_merging: bool,

    pub show_lyrics_dialog: bool,
    pub show_remove_targets_dialog: bool,

    pub show_success_dialog: bool,
    pub show_error_dialog: bool,
    pub error_message: String,
}

impl GUIState {
    pub fn new() -> Self {
        Self {
            logger: GUILogger::new(),

            font_id: None,
            font_id_jp: None,

            dsc_inputs: Vec::new(),
            plaintext_inputs: Vec::new(),
            subtitle_inputs: Vec::new(),
            remove_targets_map: Vec::new(),
            output: String::new(),
            game: Game::FutureTone,

            selected_game_index: 0,

            selected_dsc_index: 0,
            selected_plaintext_index: 0,
            selected_subtitle_index: 0,

            has_challenge_time: false,
            selected_difficulty_index: 0,
            selected_difficulty: None,
            pv_id: 0,
            challenge_time_start: "00:00.000".to_string(),
            challenge_time_end: "00:00.000".to_string(),
            english_lyrics: false,
            max_lyric_length: 75,

            is_merging: false,

            show_lyrics_dialog: false,
            show_remove_targets_dialog: false,

            show_success_dialog: false,
            show_error_dialog: false,
            error_message: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.dsc_inputs.clear();
        self.plaintext_inputs.clear();
        self.subtitle_inputs.clear();
        self.remove_targets_map.clear();
        self.output.clear();
        self.game = Game::FutureTone;

        self.selected_game_index = 0;

        self.selected_dsc_index = 0;
        self.selected_plaintext_index = 0;
        self.selected_subtitle_index = 0;

        self.has_challenge_time = false;
        self.selected_difficulty_index = 0;
        self.selected_difficulty = None;
        self.pv_id = 0;
        self.challenge_time_start = "00:00.000".to_string();
        self.challenge_time_end = "00:00.000".to_string();
        self.english_lyrics = false;
        self.max_lyric_length = 75;

        self.is_merging = false;

        self.logger.reset();

        self.show_lyrics_dialog = false;
        self.show_remove_targets_dialog = false;

        self.show_success_dialog = false;
        self.show_error_dialog = false;
        self.error_message.clear();
    }

    pub fn add_dsc_input(&mut self, input: String) {
        if self.dsc_inputs.contains(&input) {
            return;
        }

        self.dsc_inputs.push(input.clone());
        self.remove_targets_map.push((input, false));
    }

    pub fn add_plaintext_input(&mut self, input: String) {
        if self.plaintext_inputs.contains(&input) {
            return;
        }

        self.plaintext_inputs.push(input.clone());
        self.remove_targets_map.push((input, false));
    }

    pub fn add_subtitle_input(&mut self, input: String) {
        self.subtitle_inputs.push(input);
    }

    pub fn remove_dsc_input(&mut self, index: usize) {
        let removed = self.dsc_inputs.remove(index);
        self.remove_targets_map
            .retain(|(input, _)| input != &removed);
    }

    pub fn remove_plaintext_input(&mut self, index: usize) {
        let removed = self.plaintext_inputs.remove(index);
        self.remove_targets_map
            .retain(|(input, _)| input != &removed);
    }

    pub fn remove_subtitle_input(&mut self, index: usize) {
        self.subtitle_inputs.remove(index);
    }

    pub fn set_output(&mut self, output: String) {
        self.output = output;
    }

    pub fn set_game(&mut self, index: usize) {
        if index > GAME_MAP.len() {
            return;
        }

        self.game = GAME_MAP[index].1;
    }

    pub fn set_difficulty(&mut self, index: usize) {
        self.selected_difficulty = ChallengeTimeDifficulty::from_integer(index);
    }

    fn get_remove_targets_inputs(&self) -> Vec<String> {
        let mut remove_targets = Vec::new();

        for (input, remove) in &self.remove_targets_map {
            if *remove {
                remove_targets.push(input.to_string());
            }
        }

        remove_targets
    }

    pub fn merge(&mut self) -> ApplicationResult {
        let mut challenge_time: Option<ChallengeTime> = None;

        if self.has_challenge_time {
            let ct = ChallengeTime::build(
                self.challenge_time_start.clone(),
                self.challenge_time_end.clone(),
                self.selected_difficulty
                    .unwrap_or(ChallengeTimeDifficulty::Easy),
            );

            if ct.is_err() {
                return Err(ct.err().unwrap());
            }

            challenge_time = Some(ct.unwrap());
        }

        let mut application = Application::new(
            self.dsc_inputs.clone(),
            self.plaintext_inputs.clone(),
            self.subtitle_inputs.clone(),
            self.get_remove_targets_inputs(),
            self.output.clone(),
            self.game,
            self.pv_id.clamp(0, 999).try_into().unwrap(),
            self.english_lyrics,
            self.max_lyric_length.clamp(0, 1000).try_into().unwrap(),
            false,
            true,
            challenge_time,
            &mut self.logger,
        );

        application.run()
    }
}
