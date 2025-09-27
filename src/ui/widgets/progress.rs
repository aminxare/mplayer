use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Gauge, StatefulWidget, Widget},
};

use crate::audio::song::Song;

pub struct ProgressBar;
pub struct  ProgressBarState<'a> {
    song: &'a Option<Song>,
}

impl<'a> ProgressBarState<'a> {
    pub fn new(song: &'a Option<Song>) -> Self {
        Self { song }
    }
}

impl<'a> StatefulWidget for &'a ProgressBar {
    type State = ProgressBarState<'a>;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        {
            let (duration, progress) = if let Some(s) = &state.song {
                (s.duration, s.progress)
            } else {
                (0.0, 0.0)
            };

            // Render progress bar
            let progress_ratio = if duration > 0.0 {
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
                    format_time(progress as u32),
                    format_time(duration as u32)
                ));

            progress_bar.render(area, buf);
        }
    }
}

// Helper function to format time in MM:SS
fn format_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", minutes, secs)
}
