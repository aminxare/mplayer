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
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    // KeyCode::Enter => {
                    //     // وقتی کاربر Enter رو می‌زنه، مسیر رو ذخیره می‌کنیم
                    //     if !state.input.is_empty() {
                    //         state.song_path = Some(PathBuf::from(state.input.clone()));
                    //         state.input.clear();
                    //         state.input_mode = InputMode::Normal;
                    //     }
                    // }
                    // KeyCode::Char(c) => {
                    //     state.input.push(c);
                    // }
                    // KeyCode::Backspace => {
                    //     state.input.pop(); // remove latest character
                    // }
                    // KeyCode::Esc => {
                    //     state.input.clear();
                    //     state.input_mode = InputMode::Normal;
                    // }
                    _ => {}
                },
            }
        }
        Ok(false)
    }
}
