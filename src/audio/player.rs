use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};
use std::cell::Cell;
use std::fs::File;
use std::io::BufReader;

use crate::audio::library::AudioSource;
use crate::audio::song::Song;
use crate::errors::MusicPlayerError;

pub struct AudioPlayer {
    sink: Sink,
    source: Box<dyn AudioSource>,
    now_playing: Cell<Option<usize>>, // index of current playing song
    _stream: OutputStream,            // نگه داشتن OutputStream برای جلوگیری از drop شدن
    is_playing: Cell<bool>,           // playback status
    volume: Cell<f32>,
}

impl AudioPlayer {
    pub fn new(source: Box<dyn AudioSource>) -> Result<Self, MusicPlayerError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        Ok(AudioPlayer {
            _stream,
            sink,
            source,
            now_playing: Cell::new(None),
            is_playing: Cell::new(false),
            volume: Cell::new(1.0),
        })
    }

    pub fn play(&self, index: Option<usize>) -> Result<(), MusicPlayerError> {
        let source = &self.source;
        self.now_playing.set(index);

        self.stop();
        if !source.get_songs().is_empty() {
            if self.now_playing.get().is_none() {
                self.now_playing.set(Some(0)) // first song of the list
            }
        } else {
            return Err(MusicPlayerError::PlaylistError(String::from(
                "Playlist is empty!",
            )));
        }

        let song = source.get_song(self.now_playing.get().unwrap()).ok_or(
            MusicPlayerError::FileNotFound(String::from("Audio file not found")),
        )?;

        let file = File::open(song.path.clone())?;
        let source = Decoder::new(BufReader::new(file))?;
        self.sink.append(source);
        self.sink.set_volume(self.volume.get());
        self.resume();
        Ok(())
    }

    /// Toggle between pause and resume
    pub fn toggle_play(&self) {
        if self.is_playing.get() {
            self.pause();
        } else {
            self.resume();
        }
    }

    pub fn pause(&self) {
        self.is_playing.set(false);
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.is_playing.set(true);
        self.sink.play();
    }

    pub fn stop(&self) {
        self.is_playing.set(false);
        self.sink.stop();
    }

    pub fn next_song(&self) -> Result<(), MusicPlayerError> {
        let songs_count = self.source.get_songs().len();
        if songs_count == 0 {
            return Ok(());
        }

        let next_idx = match self.now_playing.get() {
            Some(idx) => (idx + 1) % songs_count,
            None => 0,
        };

        self.play(Some(next_idx))
    }

    pub fn previous_song(&self) -> Result<(), MusicPlayerError> {
        let songs_count = self.source.get_songs().len();
        if songs_count == 0 {
            return Ok(());
        }

        let prev_idx = match self.now_playing.get() {
            Some(idx) => {
                if idx == 0 {
                    songs_count - 1
                } else {
                    idx - 1
                }
            }
            None => 0,
        };

        self.play(Some(prev_idx))
    }

    pub fn increase_volume(&self) {
        let new_vol = (self.volume.get() + 0.1).min(2.0);
        self.volume.set(new_vol);
        self.sink.set_volume(new_vol);
    }

    pub fn decrease_volume(&self) {
        let new_vol = (self.volume.get() - 0.1).max(0.0);
        self.volume.set(new_vol);
        self.sink.set_volume(new_vol);
    }

    pub fn seek_forward(&self) {
        let current_pos = self.sink.get_pos();
        let new_pos = current_pos + std::time::Duration::from_secs(5);
        let _ = self.sink.try_seek(new_pos);
    }

    pub fn seek_backward(&self) {
        let current_pos = self.sink.get_pos();
        let new_pos = current_pos.saturating_sub(std::time::Duration::from_secs(5));
        let _ = self.sink.try_seek(new_pos);
    }

    pub fn current_song(&self) -> Option<Song> {
        if let Some(idx) = self.now_playing.get() {
            let mut song = self.source.get_song(idx).cloned();
            if let Some(ref mut s) = song {
                s.progress = self.sink.get_pos().as_secs_f32();
            }
            song
        } else {
            None
        }
    }

    pub fn check_and_play_next(&self) -> Result<(), MusicPlayerError> {
        if self.is_playing.get() && self.sink.empty() {
            self.next_song()?;
        }
        Ok(())
    }

    pub fn get_songs(&self) -> &[Song] {
        let source = &self.source;
        source.get_songs()
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.get()
    }
}
