use id3::{Tag, TagLike};
use std::path::PathBuf;

/// Entity for song information
#[derive(Clone, Debug)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub duration: u32, // in seconds
    pub progress: u32, // in seconds
    pub path: PathBuf,
}

impl Song {
    /// creates new song
    /// path_str is audio file path
    /// (for example: path_str = /tmp/my_song.mp3)
    pub fn new(path_str: PathBuf) -> anyhow::Result<Self> {
        let tag = Tag::read_from_path(&path_str)?;
        let title = String::from(tag.title().unwrap_or("Unkown"));
        let artist = String::from(tag.artist().unwrap_or("Unkown"));
        let duration = tag.duration().unwrap_or(0);

        Ok(Self {
            title,
            artist,
            duration,
            progress: 0,
            path: path_str.clone(),
        })
    }
}
