use crate::{
    audio::{library::MusicLibrary, player::AudioPlayer},
    errors::MusicPlayerError,
};
use std::path::PathBuf;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct AppState {
    pub music_libray: MusicLibrary,
    pub audio_player: AudioPlayer,
    pub is_playing: bool,       // playback status
    pub input_mode: InputMode,  // mode of programe
    pub dir_path: PathBuf,      // Path to the directory where the audio files are located.
    pub status_message: String, // this message will print on status bar
}

impl AppState {
    /// create new AppState
    /// dir_path: Path to the directory where the audio files are located.
    pub fn new(dir_path: String) -> anyhow::Result<Self, MusicPlayerError> {
        let mut music_libray = MusicLibrary::new();
        let dir_path = PathBuf::from(dir_path);
        let mut status_message = String::new();

        if let Err(e) = music_libray.scan_directory(&dir_path) {
            if let MusicPlayerError::FileNotFound(error_message) = e {
                status_message = error_message;
            }
        }

        let player = AudioPlayer::new()?;
        //
        //
        // Todo:
        //     get current song
        //     next song
        //     previous song
        //     in library
        //
        //

        Ok(Self {
            audio_player: player,
            music_libray,
            is_playing: false,
            input_mode: InputMode::Normal,
            dir_path,
            status_message,
        })
    }
}
