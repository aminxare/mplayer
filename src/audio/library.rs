//! Discovery and indexing of playable music files.

use anyhow::Result;

use crate::audio::song::Song;
use crate::errors::MusicPlayerError;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

/// Common behavior for any source that can provide songs to the player.
pub trait AudioSource {
    /// Returns the song at the given index if it exists.
    fn get_song(&self, song_id: usize) -> Option<&Song>;
    /// Returns the full list of available songs.
    fn get_songs(&self) -> &[Song];
    /// Finds songs whose titles contain the provided text.
    fn search_title<'a>(&'a self, title: &'a str) -> Vec<&'a Song>;
}

/// In-memory collection of songs discovered from a directory.
#[derive(Default)]
pub struct MusicLibrary {
    songs: Vec<Song>,
}

impl MusicLibrary {
    /// Creates an empty library ready to be populated.
    pub fn new() -> Self {
        MusicLibrary { songs: Vec::new() }
    }

    /// Scans a directory for supported audio files and stores them in the library.
    pub fn scan_directory(&mut self, dir_path: &PathBuf) -> Result<(), MusicPlayerError> {
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
