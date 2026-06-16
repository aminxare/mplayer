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

pub struct UI {
    pub list_state: PlayListState,
    pub status_message: Rc<RefCell<String>>,
}

impl UI {
    pub fn new(audio_player: &AudioPlayer, status_message: Rc<RefCell<String>>) -> Self {
        let list_state = PlayListState::new(audio_player.get_songs().to_vec());

        Self {
            list_state,
            status_message,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, current_song: Option<Song>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main (Playlist + Info)
                Constraint::Length(3), // Progress
                Constraint::Length(3), // Controls
                Constraint::Length(1), // Status Bar
            ])
            .split(frame.area());

        // Header
        let header = Paragraph::new(" 🎵 MPlayer - Your Terminal Music Player ")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded));
        frame.render_widget(header, chunks[0]);

        // Main Content (Playlist + Now Playing)
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(chunks[1]);

        // Playlist
        frame.render_stateful_widget(PlayList, main_chunks[0], &mut self.list_state);

        // Now Playing Info
        frame.render_widget(
            &widgets::music_player::MusicPlayer { song: current_song.clone() },
            main_chunks[1],
        );

        // Progress Bar
        frame.render_stateful_widget(
            &widgets::progress::ProgressBar,
            chunks[2],
            &mut widgets::progress::ProgressBarState::new(&current_song),
        );

        // Controls Help
        let controls = Paragraph::new(" [q] Quit | [j/k] Navigate | [Enter] Play | [p/c] Pause/Resume ")
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded).title(" Controls "));
        frame.render_widget(controls, chunks[3]);

        // Status Bar
        frame.render_stateful_widget(
            StatusBar,
            chunks[4],
            &mut StatusbarState {
                message: self.status_message.borrow().clone(),
            },
        );
    }
}
