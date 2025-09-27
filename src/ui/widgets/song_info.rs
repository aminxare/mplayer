use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::audio::song::Song;

pub struct SongInfo;
pub struct SongInfoState<'a> {
    song: &'a Option<Song>,
}

impl<'a> SongInfoState<'a> {
    pub fn new(song: &'a Option<Song>) -> Self {
        Self { song }
    }
}

impl<'a> StatefulWidget for &'a SongInfo {
    type State = SongInfoState<'a>;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let song = &state.song;
        let title = match song {
            Some(s) => s.title.clone(),
            None => String::new(),
        };
        let artist = match song {
            Some(s) => s.artist.clone(),
            None => String::new(),
        };

        let song_info = Paragraph::new(vec![
            Line::from(vec![Span::styled(
                title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![Span::styled(
                artist,
                Style::default().fg(Color::LightMagenta),
            )]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(" Now Playing ")
                .title_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .padding(Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });

        song_info.render(area, buf);
    }
}
