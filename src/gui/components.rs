use std::path::Path;

use imgui::{ComboBox, Condition, ListBox, PopupModal, Selectable, Ui, Window};
use rfd::FileDialog;

use crate::common::GAME_MAP;

use super::{state::GUIState, utils::rgba_to_imvec};

pub struct GUIComponents;

impl GUIComponents {
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) fn draw(&mut self, ui: &Ui, state: &mut GUIState) {
        if let Some(font_id) = state.font_id {
            let _inter = ui.push_font(font_id);
        }

        Window::new("DSC Merger")
            .position([0.0, 0.0], Condition::FirstUseEver)
            .size([600.0, 500.0], Condition::FirstUseEver)
            .resizable(false)
            .collapsible(false)
            .movable(false)
            .title_bar(false)
            .bring_to_front_on_focus(false)
            .build(ui, || {
                self.draw_game_selection_combo_box(ui, state);
                self.draw_vertical_spacing(ui, 10.0);
                self.draw_input_columns(ui, state);
                self.draw_subtitle_components(ui, state);
                self.draw_vertical_spacing(ui, 10.0);
                self.draw_challenge_time_components(ui, state);
                self.draw_vertical_spacing(ui, 10.0);
                self.draw_output_path_field(ui, state);
                self.draw_vertical_spacing(ui, 10.0);
                self.draw_footer(ui, state);
                self.draw_vertical_spacing(ui, 10.0);

                self.draw_status_bar(ui, state);

                self.draw_success_dialog(ui, state);
                self.draw_error_dialog(ui, state);
                self.draw_lyrics_dialog(ui, state);
            });
    }

    fn draw_vertical_spacing(&self, ui: &Ui, height: f32) {
        ui.dummy([0.0, height]);
    }

    fn draw_game_selection_combo_box(&mut self, ui: &Ui, state: &mut GUIState) {
        let items = GAME_MAP.iter().map(|(name, _)| *name).collect::<Vec<_>>();

        self.draw_left_label(ui, "Target game:");

        let iw = ui.push_item_width(-1.0);

        ComboBox::new("##game_selection_combo_box")
            .preview_value(items[state.selected_game_index])
            .build(ui, || {
                for (i, item) in items.iter().enumerate() {
                    let is_selected = state.selected_game_index == i;
                    let item = Selectable::new(item).selected(is_selected);
                    if item.build(ui) {
                        state.selected_game_index = i;
                        state.set_game(i);
                    }
                }
            });

        iw.pop(ui);
    }

    fn draw_input_columns(&mut self, ui: &Ui, state: &mut GUIState) {
        ui.columns(3, "input_columns", false);

        self.draw_dsc_list_column(ui, state);
        ui.next_column();

        self.draw_plaintext_list_column(ui, state);
        ui.next_column();

        self.draw_subtitle_list_column(ui, state);
        ui.next_column();

        ui.columns(1, "input_columns", false);
    }

    fn draw_dsc_list_column(&mut self, ui: &Ui, state: &mut GUIState) {
        ui.text("DSC files:");

        let iw = ui.push_item_width(-1.0);

        ListBox::new("##dsc_files_list").build(ui, || {
            for (i, dsc) in state.dsc_inputs.iter().enumerate() {
                let is_selected = state.selected_dsc_index == i;
                let item_text = self.filename_from_path(dsc);
                let item = Selectable::new(item_text).selected(is_selected);
                if item.build(ui) {
                    state.selected_dsc_index = i;
                }
            }
        });

        iw.pop(ui);

        if ui.button("+ DSC") {
            let file = FileDialog::new()
                .add_filter("DSC files", &["dsc"])
                .pick_file();

            if let Some(file) = file {
                state.add_dsc_input(file.as_path().to_str().unwrap().to_string());
            }
        }

        if state.dsc_inputs.len() > 0 {
            ui.same_line();

            if ui.button("- DSC") {
                state.dsc_inputs.remove(state.selected_dsc_index);
            }
        }
    }

    fn draw_plaintext_list_column(&mut self, ui: &Ui, state: &mut GUIState) {
        ui.text("Plaintext files:");

        let iw = ui.push_item_width(-1.0);

        ListBox::new("##plaintext_files_list").build(ui, || {
            for (i, plaintext) in state.plaintext_inputs.iter().enumerate() {
                let is_selected = state.selected_plaintext_index == i;
                let item_text = self.filename_from_path(plaintext);
                let item = Selectable::new(item_text).selected(is_selected);
                if item.build(ui) {
                    state.selected_plaintext_index = i;
                }
            }
        });

        iw.pop(ui);

        if ui.button("+ Plaintext") {
            let file = FileDialog::new().pick_file();

            if let Some(file) = file {
                state.add_plaintext_input(file.as_path().to_str().unwrap().to_string());
            }
        }

        if state.plaintext_inputs.len() > 0 {
            ui.same_line();

            if ui.button("- Plaintext") {
                state
                    .plaintext_inputs
                    .remove(state.selected_plaintext_index);
            }
        }
    }

    fn draw_subtitle_list_column(&mut self, ui: &Ui, state: &mut GUIState) {
        ui.text("Subtitles:");

        let iw = ui.push_item_width(-1.0);

        ListBox::new("##subtitle_files_list").build(ui, || {
            for (i, subtitle) in state.subtitle_inputs.iter().enumerate() {
                let is_selected = state.selected_subtitle_index == i;
                let item_text = self.filename_from_path(subtitle);
                let item = Selectable::new(item_text).selected(is_selected);
                if item.build(ui) {
                    state.selected_subtitle_index = i;
                }
            }
        });

        iw.pop(ui);

        if ui.button("+ Subtitle") {
            let file = FileDialog::new()
                .add_filter("Subtitle files", &["srt", "ass", "ssa"])
                .pick_file();

            if let Some(file) = file {
                state.add_subtitle_input(file.as_path().to_str().unwrap().to_string());
            }
        }

        if state.subtitle_inputs.len() > 0 {
            ui.same_line();

            if ui.button("- Subtitle") {
                state.subtitle_inputs.remove(state.selected_subtitle_index);
            }
        }
    }

    fn draw_challenge_time_components(&mut self, ui: &Ui, state: &mut GUIState) {
        self.draw_has_challenge_time_checkbox(ui, state);

        if state.has_challenge_time {
            self.draw_vertical_spacing(ui, 10.0);

            ui.columns(3, "challenge_time_columns", false);

            self.draw_difficulty_combo_box(ui, state);
            ui.next_column();

            self.draw_challenge_time_start_field(ui, state);
            ui.next_column();

            self.draw_challenge_time_end_field(ui, state);
            ui.next_column();

            ui.columns(1, "challenge_time_columns", false);
        }
    }

    fn draw_has_challenge_time_checkbox(&mut self, ui: &Ui, state: &mut GUIState) {
        ui.checkbox(
            "This chart has Challenge Time",
            &mut state.has_challenge_time,
        );
    }

    fn draw_difficulty_combo_box(&mut self, ui: &Ui, state: &mut GUIState) {
        let items = &["Easy", "Normal"];

        self.draw_left_label(ui, "Chart difficulty:");

        let iw = ui.push_item_width(-1.0);

        ComboBox::new("##difficulty_combo_box")
            .preview_value(items[state.selected_difficulty_index])
            .build(ui, || {
                for (i, item) in items.iter().enumerate() {
                    let is_selected = state.selected_difficulty_index == i;
                    let item = Selectable::new(item).selected(is_selected);
                    if item.build(ui) {
                        state.selected_difficulty_index = i;
                        state.set_difficulty(i);
                    }
                }
            });

        iw.pop(ui);
    }

    fn draw_challenge_time_start_field(&mut self, ui: &Ui, state: &mut GUIState) {
        self.draw_left_label(ui, "Start time:");

        let iw = ui.push_item_width(-1.0);

        ui.input_text("##challenge_start_field", &mut state.challenge_time_start)
            .build();

        iw.pop(ui);
    }

    fn draw_challenge_time_end_field(&mut self, ui: &Ui, state: &mut GUIState) {
        self.draw_left_label(ui, "End time:");

        let iw = ui.push_item_width(-1.0);

        ui.input_text("##challenge_end_field", &mut state.challenge_time_end)
            .build();

        iw.pop(ui);
    }

    fn draw_subtitle_components(&mut self, ui: &Ui, state: &mut GUIState) {
        if state.subtitle_inputs.len() > 0 {
            self.draw_vertical_spacing(ui, 10.0);
            ui.columns(3, "subtitle_columns", false);
            self.draw_pv_id_field(ui, state);
            ui.next_column();
            self.draw_english_lyrics_checkbox(ui, state);
            ui.next_column();
            self.draw_max_lyric_length_field(ui, state);
            ui.next_column();
            ui.columns(1, "subtitle_columns", false);
        }
    }

    fn draw_pv_id_field(&mut self, ui: &Ui, state: &mut GUIState) {
        self.draw_left_label(ui, "PV ID:");

        let iw = ui.push_item_width(-1.0);

        ui.input_int("##pv_id_field", &mut state.pv_id).build();

        iw.pop(ui);
    }

    fn draw_english_lyrics_checkbox(&mut self, ui: &Ui, state: &mut GUIState) {
        ui.checkbox("English lyrics", &mut state.english_lyrics);
    }

    fn draw_max_lyric_length_field(&mut self, ui: &Ui, state: &mut GUIState) {
        self.draw_left_label(ui, "Max lyric length:");

        let iw = ui.push_item_width(-1.0);

        ui.input_int("##max_lyric_length_field", &mut state.max_lyric_length)
            .build();

        iw.pop(ui);
    }

    fn draw_output_path_field(&mut self, ui: &Ui, state: &mut GUIState) {
        self.draw_left_label(ui, "Output path:");

        ui.input_text("##output_path_field", &mut state.output)
            .build();

        ui.same_line();

        if ui.button("Browse") {
            let path = FileDialog::new().save_file();

            if let Some(path) = path {
                state.set_output(path.as_path().to_str().unwrap().to_string());
            }
        }
    }

    fn draw_footer(&mut self, ui: &Ui, state: &mut GUIState) {
        ui.separator();
        self.draw_vertical_spacing(ui, 10.0);

        if !state.is_merging {
            if ui.button_with_size("Merge!", [100.0, 30.0]) {
                state.logger.reset();

                match state.merge() {
                    Ok(_) => {
                        state.is_merging = false;
                        state.show_success_dialog = true;
                    }
                    Err(e) => {
                        state.is_merging = false;
                        state.show_error_dialog = true;
                        state.error_message = e.to_string();
                    }
                }
            }

            ui.same_line();

            if ui.button_with_size("Reset", [100.0, 30.0]) {
                state.reset();
            }
        }

        if state.logger.lyrics.len() > 0 {
            ui.same_line();

            if ui.button_with_size("View lyrics", [100.0, 30.0]) {
                state.show_lyrics_dialog = true;
            }
        }
    }

    fn draw_status_bar(&mut self, ui: &Ui, state: &mut GUIState) {
        let x = 0.0;
        let y = ui.window_size()[1] - 28.0;

        let draw_list = ui.get_window_draw_list();
        draw_list
            .add_rect(
                [x, y],
                ui.window_size(),
                rgba_to_imvec(0x26, 0x26, 0x26, 1.0),
            )
            .filled(true)
            .build();

        ui.set_cursor_pos([x, y]);

        ui.separator();
        ui.text(state.logger.log.clone());
    }

    fn draw_success_dialog(&mut self, ui: &Ui, state: &mut GUIState) {
        if !state.show_success_dialog {
            return;
        }

        let modal = PopupModal::new("Yay!")
            .always_auto_resize(true)
            .resizable(false)
            .collapsible(false);

        modal.build(ui, || {
            ui.text("DSC files merged successfully!");

            self.draw_vertical_spacing(ui, 5.0);

            if self.draw_centered_button(ui, "OK") {
                state.show_success_dialog = false;
            }
        });

        ui.open_popup("Yay!");
    }

    fn draw_error_dialog(&mut self, ui: &Ui, state: &mut GUIState) {
        if !state.show_error_dialog {
            return;
        }

        let modal = PopupModal::new("Error")
            .always_auto_resize(true)
            .resizable(false)
            .collapsible(false);

        modal.build(ui, || {
            ui.text(&state.error_message);

            self.draw_vertical_spacing(ui, 5.0);

            if self.draw_centered_button(ui, "OK") {
                state.show_error_dialog = false;
            }
        });

        ui.open_popup("Error");
    }

    fn draw_lyrics_dialog(&mut self, ui: &Ui, state: &mut GUIState) {
        if !state.show_lyrics_dialog {
            return;
        }

        let w = ui.window_size()[0] * 0.85;
        let h = if state.logger.problematic_lyrics_lines.len() > 0 {
            ui.window_size()[1] * 0.85
        } else {
            ui.window_size()[1] * 0.5
        };

        let x = (ui.window_size()[0] - w) / 2.0;
        let y = (ui.window_size()[1] - h) / 2.0;

        let window = Window::new("mod_pv_db.txt lyrics entry")
            .always_auto_resize(false)
            .resizable(false)
            .collapsible(false)
            .size([w, h], Condition::Always)
            .position([x, y], Condition::Always);

        window.build(ui, || {
            ui.text("Put the following lines inside your mod_pv_db.txt:");

            self.draw_vertical_spacing(ui, 5.0);

            let mut lyrics = state.logger.lyrics.join("\n");

            let noto_sans_jp = ui.push_font(state.font_id_jp.unwrap());

            let iw = ui.push_item_width(-1.0);

            ui.input_text_multiline("##lyrics", &mut lyrics, [0.0, 0.0])
                .read_only(true)
                .build();

            iw.pop(ui);

            noto_sans_jp.pop();

            if state.logger.problematic_lyrics_lines.len() > 0 {
                self.draw_vertical_spacing(ui, 5.0);

                ui.text("Warnings:");

                self.draw_vertical_spacing(ui, 5.0);

                let iw = ui.push_item_width(-1.0);

                let problematic_lines = state
                    .logger
                    .problematic_lyrics_lines
                    .iter()
                    .map(|(line, max, actual)| {
                        format!(
                            "Line {} exceeds recommended byte length of {}. Actual length: {}",
                            line, max, actual
                        )
                    })
                    .collect::<Vec<String>>();

                let mut current_item = 0;

                ListBox::new("##lyrics_warnings").build_simple(
                    ui,
                    &mut current_item,
                    &problematic_lines,
                    &|item| std::borrow::Cow::Borrowed(item),
                );

                iw.pop(ui);
            }

            self.draw_vertical_spacing(ui, 5.0);

            if self.draw_centered_button(ui, "Close") {
                state.show_lyrics_dialog = false;
            }
        });
    }

    fn draw_centered_button(&mut self, ui: &Ui, text: &str) -> bool {
        let w = text.len() as f32 * 16.0 + 20.0;
        let x = (ui.content_region_avail()[0] - w) / 2.0;

        ui.set_cursor_pos([x, ui.cursor_pos()[1]]);

        let result = ui.button_with_size(text, [w, 20.0]);

        result
    }

    fn draw_left_label(&mut self, ui: &Ui, text: &str) {
        self.draw_text_with_top_padding(ui, text, 3.0);
    }

    fn draw_text_with_top_padding(&mut self, ui: &Ui, text: &str, padding: f32) {
        let previous_cursor_pos = ui.cursor_pos();
        let y = previous_cursor_pos[1] + padding;

        ui.set_cursor_pos([previous_cursor_pos[0], y]);
        ui.text(text);
        ui.same_line();

        let mut current_cursor_pos = ui.cursor_pos();
        current_cursor_pos[1] = previous_cursor_pos[1];
        ui.set_cursor_pos(current_cursor_pos);
    }

    fn filename_from_path(&self, path: &str) -> String {
        let path = Path::new(path);
        let filename = path.file_name().unwrap().to_str().unwrap();
        filename.to_string()
    }
}
