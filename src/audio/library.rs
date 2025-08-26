use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

/// Entity for song information
#[derive(Clone, Debug)]
pub struct Song {
    pub path: PathBuf,
    pub file_name: String,
}

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
    pub fn scan_directory(&mut self, dir_path: &str) -> anyhow::Result<()> {
        let path = Path::new(dir_path);
        if !path.is_dir() {
            return Err(anyhow::anyhow!("Path is not a directory: {}", dir_path));
        }

        self.songs.clear(); // Clear previous list
        for entry in fs::read_dir(path)? {
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
                let file_name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_default();
                return Some(Song { path, file_name });
            }
        }
    }
    None
}
