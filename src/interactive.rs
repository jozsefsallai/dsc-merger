use requestty::{prompt_one, Question};

use crate::{application::Application, common::Game};

const GAME_MAP: [(&'static str, Game); 4] = [
    (
        "Project Diva AFT / Future Tone / Mega Mix / Mega Mix+",
        Game::FutureTone,
    ),
    (
        "Project Diva F / Dreamy Theater 2nd / Dreamy Theater Extend / f",
        Game::F,
    ),
    ("Project Diva F 2nd", Game::F2nd),
    ("Project Diva X", Game::X),
];

struct InputFiles {
    dsc: Vec<String>,
    plaintext: Vec<String>,
    subtitle: Vec<String>,
}

impl InputFiles {
    fn new() -> Self {
        Self {
            dsc: Vec::new(),
            plaintext: Vec::new(),
            subtitle: Vec::new(),
        }
    }

    fn add_dsc(&mut self, path: String) {
        self.dsc.push(path);
    }

    fn add_plaintext(&mut self, path: String) {
        self.plaintext.push(path);
    }

    fn add_subtitle(&mut self, path: String) {
        self.subtitle.push(path);
    }
}

pub struct InteractiveTUI;

impl InteractiveTUI {
    fn new() -> Self {
        Self {}
    }

    pub fn start() {
        let tui = InteractiveTUI::new();

        let game = tui.prompt_game();
        let input_files = tui.prompt_input_files();

        let mut pv_id = 0;
        let mut english_lyrics = false;
        let mut max_lyric_length = 75;

        if input_files.subtitle.len() > 0 {
            println!("It looks like you're adding lyrics. To make things easier, tell me a few things about your song!");

            pv_id = tui.prompt_pv_id();
            english_lyrics = tui.prompt_english_lyrics();
            max_lyric_length = tui.prompt_max_lyric_length();
        }

        let verbose = tui.prompt_verbose();
        let output = tui.prompt_output();

        let application = Application::new(
            input_files.dsc,
            input_files.plaintext,
            input_files.subtitle,
            output,
            game,
            pv_id,
            english_lyrics,
            max_lyric_length,
            false,
            verbose,
        );

        match application.run() {
            Ok(_) => println!("Done!"),
            Err(e) => println!("Error: {}", e),
        };

        tui.display_press_enter_to_exit();
    }

    fn prompt_game(&self) -> Game {
        let question = Question::select("game")
            .message("Select the game the chart is designed for.")
            .choices(GAME_MAP.map(|(name, _)| name))
            .default_separator()
            .choice("Abort")
            .build();

        let answer = prompt_one(question).unwrap();

        match answer.as_list_item() {
            Some(item) => match item.text.as_str() {
                "Abort" => {
                    println!("Aborted.");
                    std::process::exit(0);
                }
                _ => {
                    let (_, game) = GAME_MAP
                        .iter()
                        .find(|(name, _)| name == &item.text)
                        .unwrap();
                    *game
                }
            },
            None => std::process::exit(-1),
        }
    }

    fn prompt_input_files(&self) -> InputFiles {
        let mut input_files = InputFiles::new();

        loop {
            let question = Question::select("action")
                .message("Select an action:")
                .choices(vec![
                    "Add DSC file",
                    "Add plaintext file",
                    "Add subtitle file",
                    "Continue",
                ])
                .default_separator()
                .choice("Abort")
                .build();

            let answer = prompt_one(question).unwrap();

            match answer.as_list_item() {
                Some(item) => match item.index {
                    0 | 1 | 2 => {
                        let question = Question::input("path")
                            .message("Enter the path to the file:")
                            .build();

                        let answer = prompt_one(question).unwrap();

                        match answer.as_string() {
                            Some(input) => match item.index {
                                0 => input_files.add_dsc(input.to_string()),
                                1 => input_files.add_plaintext(input.to_string()),
                                2 => input_files.add_subtitle(input.to_string()),
                                _ => {}
                            },
                            None => {}
                        }
                    }
                    3 => {
                        break;
                    }
                    5 => {
                        println!("Aborted.");
                        std::process::exit(0);
                    }
                    _ => unreachable!(),
                },
                None => std::process::exit(-1),
            }
        }

        input_files
    }

    fn prompt_pv_id(&self) -> u16 {
        let question = Question::input("pv_id")
            .message("Enter the ID of your PV:")
            .default("000")
            .build();

        let answer = prompt_one(question).unwrap();

        match answer.as_string() {
            Some(input) => match input.parse::<u16>() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid PV ID.");
                    std::process::exit(-1);
                }
            },
            None => std::process::exit(-1),
        }
    }

    fn prompt_english_lyrics(&self) -> bool {
        let question = Question::confirm("english_lyrics")
            .message("Do the subtitles contain English lyrics (lyric_en)?")
            .default(false)
            .build();

        let answer = prompt_one(question).unwrap();

        match answer.as_bool() {
            Some(input) => input,
            None => std::process::exit(-1),
        }
    }

    fn prompt_max_lyric_length(&self) -> u16 {
        let question = Question::input("max_lyric_length")
            .message("What is the maximum allowed length for a lyric line?")
            .default("75")
            .build();

        let answer = prompt_one(question).unwrap();

        match answer.as_string() {
            Some(input) => match input.parse::<u16>() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid lyric length.");
                    std::process::exit(-1);
                }
            },
            None => std::process::exit(-1),
        }
    }

    fn prompt_verbose(&self) -> bool {
        let question = Question::confirm("verbose")
            .message("Do you want to see verbose output?")
            .default(false)
            .build();

        let answer = prompt_one(question).unwrap();

        match answer.as_bool() {
            Some(input) => input,
            None => std::process::exit(-1),
        }
    }

    fn prompt_output(&self) -> String {
        let question = Question::input("output")
            .message("Finally, enter the path to the output file:")
            .build();

        let answer = prompt_one(question).unwrap();

        match answer.as_string() {
            Some(input) => input.to_string(),
            None => std::process::exit(-1),
        }
    }

    fn display_press_enter_to_exit(&self) {
        println!("Press ENTER to exit...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
