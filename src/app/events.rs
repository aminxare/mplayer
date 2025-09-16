use crate::app::state::{AppState, InputMode};
use crossterm::event::{Event, KeyCode, KeyEvent};
use std::path::PathBuf;

pub fn handle_events(event: Event, state: &mut AppState) -> anyhow::Result<bool> {
    if let Event::Key(key) = event {
        match state.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') => return Ok(true), // exit
                // KeyCode::Char('i') => state.input_mode = InputMode::Editing,
                KeyCode::Char('p') => {
                    state.is_playing = !state.is_playing;
                    if state.is_playing {
                        state
                            .audio_player
                            .play()?;
                    } else {
                        state.audio_player.pause();
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
