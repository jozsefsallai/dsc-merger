mod components;
mod gui_logger;
mod renderer;
mod state;
mod utils;

pub struct GUI;

impl GUI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let mut gui_renderer = renderer::GUIRenderer::new();
        gui_renderer.run();
    }
}
