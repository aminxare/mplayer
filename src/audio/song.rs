use id3::{Tag, TagLike};
use std::path::PathBuf;
use std::time::Duration;

/// Entity for song information
#[derive(Clone, Debug)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub duration: f32, // in seconds
    pub progress: f32, // in seconds
    pub path: PathBuf,
}

impl Song {
    /// creates new song
    /// path_str is audio file path
    /// (for example: path_str = /tmp/my_song.mp3)
    pub fn new(path_str: PathBuf) -> anyhow::Result<Self> {
        let tag = Tag::read_from_path(&path_str)?;
        let title = String::from(tag.title().unwrap_or("Unknown"));
        let artist = String::from(tag.artist().unwrap_or("Unknown"));
        
        // Try to get duration from mp3-duration first for better accuracy
        let duration = if let Some(ext) = path_str.extension() {
            if ext.to_string_lossy().to_lowercase() == "mp3" {
                mp3_duration::from_path(&path_str)
                    .map(|d| d.as_secs_f32())
                    .unwrap_or_else(|_| {
                        // Fallback to ID3 tag if mp3-duration fails
                        tag.duration()
                            .map(|d| Duration::from_millis(d as u64).as_secs_f32())
                            .unwrap_or(0.0)
                    })
            } else {
                // For non-mp3 files (like wav), try ID3 tag or default to 0
                tag.duration()
                    .map(|d| Duration::from_millis(d as u64).as_secs_f32())
                    .unwrap_or(0.0)
            }
        } else {
            0.0
        };

        Ok(Self {
            title,
            artist,
            duration,
            progress: 0.0,
            path: path_str.clone(),
        })
    }
}
