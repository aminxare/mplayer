use crate::{
    audio::song::Song,
    ui::widgets::{
        progress::{ProgressBar, ProgressBarState},
        song_info::{SongInfo, SongInfoState},
    },
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{StatefulWidget, Widget},
};

pub struct MusicPlayer {
    pub song: Option<Song>,
}

impl Widget for &MusicPlayer {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(0),    // Song info
                    Constraint::Length(1), // Progress bar
                ]
                .as_ref(),
            )
            .split(area);

        SongInfo.render(chunks[0], buf, &mut SongInfoState::new(&self.song));
        ProgressBar.render(chunks[1], buf, &mut ProgressBarState::new(&self.song));
    }
}
