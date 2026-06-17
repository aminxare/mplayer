use crate::app::{App, InputMode};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};

impl App {
    pub fn handle_events(&mut self, event: Event) -> Result<bool> {
        if let Event::Key(key) = event {
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(true), // exit
                    // KeyCode::Char('i') => state.input_mode = InputMode::Editing,
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.ui.list_state.select_next();
                    }

                    KeyCode::Up | KeyCode::Char('k') => {
                        self.ui.list_state.select_previous();
                    }

                    KeyCode::Enter => {
                        let selected = self.ui.list_state.selected();
                        self.audio_player.play(selected)?;
                    }

                    KeyCode::Char('p') | KeyCode::Char('c') => {
                        self.audio_player.toggle_play();
                    }

                    KeyCode::Right | KeyCode::Char('d') => {
                        self.audio_player.seek_forward();
                    }

                    KeyCode::Left | KeyCode::Char('a') => {
                        self.audio_player.seek_backward();
                    }

                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        self.audio_player.increase_volume();
                    }

                    KeyCode::Char('-') | KeyCode::Char('_') => {
                        self.audio_player.decrease_volume();
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    // TODO: implement editing logic later
                    _ => {}
                },
            }
        }
        Ok(false)
    }
}
