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
}

impl AudioPlayer {
    pub fn new(source: Box<dyn AudioSource>) -> anyhow::Result<Self, MusicPlayerError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        Ok(AudioPlayer {
            _stream,
            sink,
            source,
            now_playing: Cell::new(None),
            is_playing: Cell::new(false),
        })
    }

    pub fn play(&self, index: Option<usize>) -> anyhow::Result<(), MusicPlayerError> {
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

    pub fn current_song(&self) -> Option<Song> {
        if let Some(idx) = self.now_playing.get() {
            self.source.get_song(idx).cloned()
        } else {
            None
        }
    }

    pub fn get_songs(&self) -> &[Song] {
        let source = &self.source;
        source.get_songs()
    }
}
