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

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    ui: UI,
    audio_player: AudioPlayer,
    is_playing: bool,                    // playback status
    input_mode: InputMode,               // mode of program
    dir_path: PathBuf,                   // Path to the directory where the audio files are located.
    status_message: Rc<RefCell<String>>, // this message will print on status bar
}

impl App {
    pub fn new(path: String) -> Result<Self> {
        let dir_path = PathBuf::from(path);
        let status_message = Rc::new(RefCell::new(String::new()));
        let mut music_libray = Box::new(MusicLibrary::new());

        if let Err(MusicPlayerError::FileNotFound(error_message)) =
            music_libray.scan_directory(&dir_path)
        {
            let mut msg = status_message.borrow_mut();
            *msg = error_message;
        }

        let audio_player = AudioPlayer::new(music_libray)?;
        let ui = UI::new(&audio_player, status_message.clone());

        let result = Self {
            ui,
            audio_player,
            dir_path,
            input_mode: InputMode::Normal,
            is_playing: false,
            status_message,
        };
        Ok(result)
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        loop {
            let csong = self.audio_player.current_song();
            terminal.draw(|f| self.ui.render(f, csong))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                let e = event::read()?;
                if self.handle_events(e)? {
                    break; // if pressed 'q' then exit
                }
            }
        }
        Ok(())
    }
}
