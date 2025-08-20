use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

struct Song {
    title: String,
    artist: String,
    duration: u32, // in seconds
    progress: u32, // in seconds
}

// pub fn draw_music_player(f: &mut Frame, area: Rect, song: &Song, is_playing: bool) {
pub fn draw_music_player(f: &mut Frame, area: Rect) {
    let song = Song {
        artist: "artist".into(),
        duration: 522,
        progress: 90,
        title: "title".into(),
    };
    let is_playing = true;

    // Main layout: vertical split for song info, progress bar, and controls
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Song info
                Constraint::Length(3), // Progress bar
                Constraint::Length(3), // Controls
            ]
            .as_ref(),
        )
        .split(area);

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

    // Progress bar
    let progress_ratio = if song.duration > 0 {
        song.progress as f64 / song.duration as f64
    } else {
        0.0
    };
    let progress_bar = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(" Progress ")
                .title_style(Style::default().fg(Color::Yellow)),
        )
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

    // Controls section
    let play_pause_symbol = if is_playing { "⏸" } else { "▶" };
    let controls = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            " ⏮ ",
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            play_pause_symbol,
            Style::default()
                .fg(Color::Green)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            " ⏭ ",
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            " 🔉 ",
            Style::default()
                .fg(Color::Blue)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        ),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(" Controls ")
            .title_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
    )
    .alignment(ratatui::layout::Alignment::Center);

    // Render widgets
    f.render_widget(song_info, chunks[0]);
    f.render_widget(progress_bar, chunks[1]);
    f.render_widget(controls, chunks[2]);
}

// Helper function to format time in MM:SS
fn format_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", minutes, secs)
}