use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Padding, Paragraph, Widget, Wrap},
};
use std::{cell::RefCell, default, rc::Rc};

use crate::audio::song::{self, Song};

pub struct MusicPlayer {
    pub song: Option<Song>,
}

impl MusicPlayer {
    fn progress_bar<'a>(&self) -> Gauge<'a> {
        let (duration, progress) = if let Some(s) = &self.song {
            (s.duration, s.progress)
        } else {
            (0, 0)
        };

        // Render progress bar
        let progress_ratio = if duration > 0 {
            progress as f64 / duration as f64
        } else {
            0.0
        };

        let progress_bar = Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent((progress_ratio * 100.0) as u16)
            .label(format!(
                "{} / {}",
                format_time(progress),
                format_time(duration)
            ));

        progress_bar
    }

    // print song information
    fn info(&self) -> Paragraph<'_> {
        let song = &self.song;
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

        song_info
    }
}

impl Widget for &MusicPlayer {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Min(0),    // Song info
                    Constraint::Length(1), // Progress bar
                ]
                .as_ref(),
            )
            .split(area);

        self.info().render(chunks[0], buf);
        self.progress_bar().render(chunks[1], buf);
    }
}

// Helper function to format time in MM:SS
fn format_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", minutes, secs)
}
