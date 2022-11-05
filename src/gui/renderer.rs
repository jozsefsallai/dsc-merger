use std::time::Instant;

use imgui::sys::*;
use imgui::{FontGlyphRanges, FontSource};
use sdl2::event::Event;

use super::utils::rgba_to_imvec;
use super::{components::GUIComponents, state::GUIState};

pub struct GUIRenderer {
    state: GUIState,
    components: GUIComponents,

    is_active: bool,
}

impl GUIRenderer {
    pub fn new() -> Self {
        Self {
            state: GUIState::new(),
            components: GUIComponents::new(),
            is_active: false,
        }
    }

    pub fn run(&mut self) {
        self.is_active = true;

        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();

        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);
        }

        let window = video
            .window("DSC Merger", 600, 500)
            .position_centered()
            .opengl()
            .allow_highdpi()
            .build()
            .unwrap();

        let _gl_context = window
            .gl_create_context()
            .expect("Failed to create GL context");
        gl::load_with(|s| video.gl_get_proc_address(s) as _);

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        self.load_fonts(&mut imgui);
        self.update_styles(&mut imgui);

        let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

        let renderer =
            imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut last_frame = Instant::now();

        loop {
            for event in event_pump.poll_iter() {
                imgui_sdl2.handle_event(&mut imgui, &event);
                if imgui_sdl2.ignore_event(&event) {
                    continue;
                }

                match event {
                    Event::Quit { .. } => {
                        self.is_active = false;
                    }
                    _ => {}
                }
            }

            imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

            let now = Instant::now();
            let delta = now - last_frame;
            let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
            last_frame = now;
            imgui.io_mut().delta_time = delta_s;

            let ui = imgui.frame();
            self.components.draw(&ui, &mut self.state);

            unsafe {
                gl::ClearColor(0.2, 0.2, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            imgui_sdl2.prepare_render(&ui, &window);
            renderer.render(ui);

            window.gl_swap_window();

            if !self.is_active {
                break;
            }
        }
    }

    fn load_fonts(&mut self, imgui: &mut imgui::Context) {
        let mut font = imgui.fonts();

        let font_id = font.add_font(&[FontSource::TtfData {
            data: include_bytes!("../../assets/inter.ttf"),
            size_pixels: 14.0,
            config: None,
        }]);

        self.state.font_id = Some(font_id);

        let glyph_ranges = FontGlyphRanges::from_slice(&[
            0x0020, 0x00FF, // Basic Latin + Latin Supplement
            0x2010, 0x205E, // Punctuations
            0x0E00, 0x0E7F, // Thai
            0x3000, 0x30FF, // Punctuations, Hiragana, Katakana
            0x31F0, 0x31FF, // Katakana Phonetic Extensions
            0xFF00, 0xFFEF, // Half-width characters
            0x4e00, 0x9FAF, // CJK Ideograms
            0,
        ]);

        let font_id_jp = font.add_font(&[FontSource::TtfData {
            data: include_bytes!("../../assets/notosansjp.ttf"),
            size_pixels: 16.0,
            config: Some(imgui::FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges,
                ..Default::default()
            }),
        }]);

        self.state.font_id_jp = Some(font_id_jp);
    }

    fn update_styles(&mut self, imgui: &mut imgui::Context) {
        imgui.style_mut().scrollbar_size = 15.0;
        imgui.style_mut().grab_min_size = 8.0;
        imgui.style_mut().window_border_size = 1.0;
        imgui.style_mut().child_border_size = 0.0;
        imgui.style_mut().popup_border_size = 1.0;
        imgui.style_mut().frame_border_size = 1.0;
        imgui.style_mut().tab_border_size = 0.0;

        imgui.style_mut().window_rounding = 0.0;
        imgui.style_mut().child_rounding = 0.0;
        imgui.style_mut().frame_rounding = 0.0;
        imgui.style_mut().popup_rounding = 0.0;
        imgui.style_mut().scrollbar_rounding = 15.0;
        imgui.style_mut().grab_rounding = 15.0;
        imgui.style_mut().tab_rounding = 0.0;

        imgui.style_mut().window_title_align = [0.50, 0.50];
        imgui.style_mut().window_rounding = 0.0;

        let bg_color = rgba_to_imvec(0x32, 0x32, 0x32, 1.0);
        let bg_color_light = rgba_to_imvec(82, 82, 85, 1.0);
        let bg_color_lighter = rgba_to_imvec(0x47, 0x47, 0x47, 1.0);

        let panel_color = rgba_to_imvec(0x29, 0x29, 0x29, 1.0);
        let panel_hover_color = rgba_to_imvec(0x24, 0x24, 0x24, 1.0);
        let panel_active_color = rgba_to_imvec(0x1F, 0x1F, 0x1F, 1.0);

        let text_color = rgba_to_imvec(0xC4, 0xC4, 0xC4, 1.0);
        let text_disabled_color = rgba_to_imvec(0x69, 0x69, 0x69, 1.0);
        let border_color = rgba_to_imvec(0x26, 0x26, 0x26, 1.0);

        let selection_color = rgba_to_imvec(0x53, 0x53, 0x53, 1.0);

        let colors = &mut imgui.style_mut().colors;
        colors[ImGuiCol_Text as usize] = text_color;
        colors[ImGuiCol_TextDisabled as usize] = text_disabled_color;
        colors[ImGuiCol_TextSelectedBg as usize] = panel_active_color;
        colors[ImGuiCol_WindowBg as usize] = bg_color;
        colors[ImGuiCol_ChildBg as usize] = bg_color;
        colors[ImGuiCol_PopupBg as usize] = bg_color;
        colors[ImGuiCol_Border as usize] = border_color;
        colors[ImGuiCol_BorderShadow as usize] = border_color;
        colors[ImGuiCol_FrameBg as usize] = panel_color;
        colors[ImGuiCol_FrameBgHovered as usize] = panel_hover_color;
        colors[ImGuiCol_FrameBgActive as usize] = panel_active_color;
        colors[ImGuiCol_TitleBg as usize] = panel_color;
        colors[ImGuiCol_TitleBgActive as usize] = panel_active_color;
        colors[ImGuiCol_TitleBgCollapsed as usize] = panel_color;
        colors[ImGuiCol_MenuBarBg as usize] = panel_color;
        colors[ImGuiCol_ScrollbarBg as usize] = panel_color;
        colors[ImGuiCol_ScrollbarGrab as usize] = bg_color_lighter;
        colors[ImGuiCol_ScrollbarGrabHovered as usize] = bg_color_light;
        colors[ImGuiCol_ScrollbarGrabActive as usize] = bg_color_light;
        colors[ImGuiCol_CheckMark as usize] = rgba_to_imvec(255, 255, 255, 1.0);
        colors[ImGuiCol_SliderGrab as usize] = panel_hover_color;
        colors[ImGuiCol_SliderGrabActive as usize] = panel_active_color;
        colors[ImGuiCol_Button as usize] = panel_color;
        colors[ImGuiCol_ButtonHovered as usize] = panel_hover_color;
        colors[ImGuiCol_ButtonActive as usize] = panel_active_color;
        colors[ImGuiCol_Header as usize] = panel_active_color;
        colors[ImGuiCol_HeaderHovered as usize] = panel_hover_color;
        colors[ImGuiCol_HeaderActive as usize] = panel_color;
        colors[ImGuiCol_Separator as usize] = border_color;
        colors[ImGuiCol_SeparatorHovered as usize] = border_color;
        colors[ImGuiCol_SeparatorActive as usize] = border_color;
        colors[ImGuiCol_ResizeGrip as usize] = bg_color;
        colors[ImGuiCol_ResizeGripHovered as usize] = panel_color;
        colors[ImGuiCol_ResizeGripActive as usize] = bg_color_light;
        colors[ImGuiCol_PlotLines as usize] = panel_active_color;
        colors[ImGuiCol_PlotLinesHovered as usize] = panel_hover_color;
        colors[ImGuiCol_PlotHistogram as usize] = panel_active_color;
        colors[ImGuiCol_PlotHistogramHovered as usize] = panel_hover_color;
        colors[ImGuiCol_DragDropTarget as usize] = bg_color;
        colors[ImGuiCol_NavHighlight as usize] = selection_color;
        colors[ImGuiCol_Tab as usize] = bg_color;
        colors[ImGuiCol_TabActive as usize] = panel_active_color;
        colors[ImGuiCol_TabUnfocused as usize] = bg_color;
        colors[ImGuiCol_TabUnfocusedActive as usize] = panel_active_color;
        colors[ImGuiCol_TabHovered as usize] = panel_hover_color;
        colors[ImGuiCol_ModalWindowDimBg as usize] = rgba_to_imvec(0, 0, 0, 0.5);
        colors[ImGuiCol_TextSelectedBg as usize] = selection_color;
    }
}
