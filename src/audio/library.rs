use crate::audio::song::Song;
use crate::errors::MusicPlayerError;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

/// Structure for handling music library
#[derive(Default)]
pub struct MusicLibrary {
    pub songs: Vec<Song>,
}

impl MusicLibrary {
    /// Create new MusicLibrary
    pub fn new() -> Self {
        MusicLibrary { songs: Vec::new() }
    }

    /// Scan a directory to find files
    pub fn scan_directory(&mut self, dir_path: &PathBuf) -> anyhow::Result<(), MusicPlayerError> {
        if !dir_path.is_dir() {
            return Err(MusicPlayerError::PlaylistError(format!(
                "Path is not a directory: {:?}",
                dir_path
            )));
        }

        self.songs.clear(); // Clear previous list
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            if let Some(song) = process_entry(&entry) {
                self.songs.push(song);
            }
        }

        Ok(())
    }

    /// get list of songs
    pub fn get_songs(&self) -> &Vec<Song> {
        &self.songs
    }
}

/// process DirEntry to check file is music
fn process_entry(entry: &DirEntry) -> Option<Song> {
    let path = entry.path();

    if path.is_file() {
        if let Some(ext) = path.extension() {
            let ext: String = ext.to_string_lossy().to_lowercase();
            if ext == "mp3" || ext == "wav" {
                return Song::new(path).ok();
            }
        }
    }
    None
}
