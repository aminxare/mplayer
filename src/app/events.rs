use crate::app::{App, InputMode};
use crossterm::event::{Event, KeyCode};

impl App {
    pub fn handle_events(&mut self, event: Event) -> anyhow::Result<bool> {
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

                    KeyCode::Char('p') => {
                        self.is_playing = !self.is_playing;
                        if self.is_playing {
                            self.audio_player.play(None)?;
                        } else {
                            self.audio_player.pause();
                        }
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
