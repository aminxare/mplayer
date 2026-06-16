use crate::{
    audio::song::Song,
    ui::widgets::song_info::{SongInfo, SongInfoState},
    ui::widgets::visualizer::Visualizer,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{StatefulWidget, Widget},
};

pub struct MusicPlayer {
    pub song: Option<Song>,
    pub is_playing: bool,
}

impl Widget for &MusicPlayer {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Song Info
                Constraint::Min(0),    // Visualizer
            ])
            .split(area);

        SongInfo.render(chunks[0], buf, &mut SongInfoState::new(&self.song));
        Visualizer { is_playing: self.is_playing }.render(chunks[1], buf);
    }
}
