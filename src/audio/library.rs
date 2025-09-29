use crate::audio::song::Song;
use crate::errors::MusicPlayerError;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

pub trait AudioSource {
    fn get_song(&self, song_id: usize) -> Option<&Song>;
    fn get_songs(&self) -> &[Song];
    fn search_title<'a>(&'a self, title: &'a str) -> Vec<&'a Song>;
}

/// Structure for handling music library
#[derive(Default)]
pub struct MusicLibrary {
    songs: Vec<Song>,
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

        // return error if there no audio file in directory
        if self.songs.is_empty() {
            return Err(MusicPlayerError::FileNotFound(format!(
                "No audio file found in {}",
                dir_path.as_path().to_str().unwrap()
            )));
        }

        Ok(())
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

impl AudioSource for MusicLibrary {
    fn get_song(&self, song_id: usize) -> Option<&Song> {
        self.songs.get(song_id)
    }

    /// get list of songs
    fn get_songs(&self) -> &[Song] {
        &self.songs
    }

    fn search_title<'a>(&'a self, title: &'a str) -> Vec<&'a Song> {
        let title = title.to_lowercase();
        let result = self
            .songs
            .iter()
            .filter(|&s| s.title.to_lowercase().contains(title.as_str()))
            .collect::<Vec<&Song>>();
        result
    }
}
