use std::path::{Path, PathBuf};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct Song {
    pub title: String,
    pub artist: String,
    pub duration: u32, // in seconds
    pub progress: u32, // in seconds
}

pub struct AppState {
    pub current_song: Option<Song>,
    pub is_playing: bool,
    pub input_mode: InputMode,
    pub song_path: Option<PathBuf>,
}

impl AppState {
    pub fn new() -> Self {
        ///TODO:
        let song_path = Path::new(std::env::current_dir().unwrap().to_str().unwrap())
            .join("./assets/sample_music/a.mp3");
        Self {
            current_song: None,
            is_playing: false,
            input_mode: InputMode::Normal,
            song_path: Some(song_path),
        }
    }
}
