use crate::{
    audio::song::Song,
    ui::widgets::song_info::{SongInfo, SongInfoState},
};
use ratatui::{
    layout::Rect,
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
        SongInfo.render(area, buf, &mut SongInfoState::new(&self.song));
    }
}
