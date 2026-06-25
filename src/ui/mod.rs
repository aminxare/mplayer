//! Terminal user interface rendering for the music player.
//!
//! This module builds the Ratatui layout, renders the playlist and playback
//! panels, and exposes the shared status message used by the application.

use std::{cell::RefCell, rc::Rc};

use crate::audio::{player::AudioPlayer, song::Song};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::widgets::{
    playlist::{PlayList, PlayListState},
    status_bar::{StatusBar, StatusbarState},
};

mod widgets;

/// Terminal UI state for the playlist and status bar.
pub struct UI {
    pub list_state: PlayListState,
    pub status_message: Rc<RefCell<String>>,
}

impl UI {
    /// Creates a new UI instance from the current audio player state.
    pub fn new(audio_player: &AudioPlayer, status_message: Rc<RefCell<String>>) -> Self {
        let list_state = PlayListState::new(audio_player.get_songs().to_vec());

        Self {
            list_state,
            status_message,
        }
    }

    /// Renders the full screen using the latest playback and UI state.
    pub fn render(&mut self, frame: &mut Frame, current_song: Option<Song>, is_playing: bool) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(frame.area());

        let header = Paragraph::new(" 🎵 MPlayer - Your Terminal Music Player ")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded));
        frame.render_widget(header, chunks[0]);

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(chunks[1]);

        frame.render_stateful_widget(PlayList, main_chunks[0], &mut self.list_state);

        frame.render_widget(
            &widgets::music_player::MusicPlayer {
                song: current_song.clone(),
                is_playing,
            },
            main_chunks[1],
        );

        frame.render_stateful_widget(
            &widgets::progress::ProgressBar,
            chunks[2],
            &mut widgets::progress::ProgressBarState::new(&current_song),
        );

        let controls = Paragraph::new(" [q] Quit | [j/k] Navigate | [Enter] Play | [p/c] Pause/Resume ")
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded).title(" Controls "));
        frame.render_widget(controls, chunks[3]);

        frame.render_stateful_widget(
            StatusBar,
            chunks[4],
            &mut StatusbarState {
                message: self.status_message.borrow().clone(),
            },
        );
    }
}
