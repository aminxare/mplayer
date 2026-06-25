//! Application state and input handling for the terminal music player.
//!
//! The app module owns the main run loop, the current UI mode, and the
//! shared status message that is displayed in the status bar.

use crate::{
    audio::{library::MusicLibrary, player::AudioPlayer},
    errors::MusicPlayerError,
    ui::UI,
};
use anyhow::Result;
use crossterm::event;
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

mod events;

/// The interaction mode currently used by the user interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Standard navigation mode.
    Normal,
    /// Text-entry mode for editing actions.
    Editing,
}

/// Top-level application controller.
///
/// The app coordinates the UI, audio backend, and input loop while the
/// player is running.
pub struct App {
    ui: UI,
    audio_player: AudioPlayer,
    input_mode: InputMode,
    dir_path: PathBuf,
    status_message: Rc<RefCell<String>>,
}

impl App {
    /// Creates a new application instance for the supplied music directory.
    ///
    /// The constructor scans the directory for playable tracks, builds the
    /// audio player, and prepares the UI state.
    pub fn new(path: String) -> Result<Self, MusicPlayerError> {
        let dir_path = PathBuf::from(&path);
        let status_message = Rc::new(RefCell::new(String::new()));
        let mut music_libray = Box::new(MusicLibrary::new());

        if let Err(error_message) = music_libray.scan_directory(&dir_path) {
            let mut msg = status_message.borrow_mut();
            *msg = error_message.to_string();
        } else {
            let mut msg = status_message.borrow_mut();
            *msg = format!("{}", path.as_str());
        }

        let audio_player = AudioPlayer::new(music_libray)?;
        let ui = UI::new(&audio_player, status_message.clone());

        let result = Self {
            ui,
            audio_player,
            dir_path,
            input_mode: InputMode::Normal,
            status_message,
        };
        Ok(result)
    }

    /// Runs the main application loop until the user exits.
    ///
    /// Each iteration updates playback, redraws the interface, and handles
    /// any pending keyboard input.
    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        loop {
            self.audio_player.check_and_play_next()?;
            let csong = self.audio_player.current_song();
            let is_playing = self.audio_player.is_playing();
            terminal.draw(|f| self.ui.render(f, csong, is_playing))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                let e = event::read()?;

                match self.handle_events(e) {
                    Ok(exit) => {
                        if exit {
                            break;
                        }
                    }
                    Err(error) => {
                        let mut s = self.status_message.borrow_mut();
                        *s = error.to_string();
                    }
                };
            }
        }
        Ok(())
    }
}
