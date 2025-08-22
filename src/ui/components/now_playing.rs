use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::state::Song;

// draw controllers and progress bar
fn draw_controllers(f: &mut Frame, area: Rect, song: &Song, is_playing: bool) {
    // Split layout horizontally: 80% for progress bar, 20% for controllers
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80), // Progress bar
            Constraint::Percentage(20), // Controllers
        ])
        .split(area);

    // Render progress bar
    let progress_ratio = if song.duration > 0 {
        song.progress as f64 / song.duration as f64
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
            format_time(song.progress),
            format_time(song.duration)
        ));

    // Render controls with vertical centering
    let play_pause_symbol = if is_playing { "⏸" } else { "▶" };
    let controls = Paragraph::new(Line::from(vec![
        Span::styled(
            " ⏮ ",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            play_pause_symbol,
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            " ⏭ ",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::NONE)
            .style(Style::default()),
    );

    // Create a vertically centered layout for controls
    let controls_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Spacer
            Constraint::Length(1), // Controls
            Constraint::Min(0),    // Spacer
        ])
        .split(chunks[1])[1]; // Use the middle chunk for controls

    f.render_widget(progress_bar, chunks[0]);
    f.render_widget(controls, controls_area);
}

fn draw_song_info(f: &mut Frame, area: Rect, song: &Song) {
    // Song info section
    let song_info = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            &song.title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            &song.artist,
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
            ),
    )
    .wrap(Wrap { trim: true });

    // Render widgets
    f.render_widget(song_info, area);
}

pub fn draw_music_player(f: &mut Frame, area: Rect, song: &Option<Song>, is_playing: bool) {
    // Main layout: vertical split for song info, progress bar, and controls
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

    // Render widgets
    if let Some(song) = song {
        draw_song_info(f, chunks[0], &song);
        draw_controllers(f, chunks[1], &song, is_playing)
    }
}

// Helper function to format time in MM:SS
fn format_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", minutes, secs)
}
