use crate::app::state::AppState;
use ratatui::Frame;

mod components;
mod views;

pub struct UI {
    // current_song: Song
}

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_ui(&self, f: &mut Frame, state: &AppState) {
        views::views(f, state);
    }
}
