use rodio::{Decoder, OutputStream, Sink};
use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

use crate::audio::library::AudioSource;
use crate::audio::song::Song;
use crate::errors::MusicPlayerError;

pub struct AudioPlayer {
    sink: Sink,
    source: Rc<RefCell<dyn AudioSource>>,
    now_playing: Cell<Option<usize>>, // index of current playing song
    _stream: OutputStream,         // نگه داشتن OutputStream برای جلوگیری از drop شدن
}

impl AudioPlayer {
    pub fn new(source: Rc<RefCell<dyn AudioSource>>) -> anyhow::Result<Self, MusicPlayerError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        Ok(AudioPlayer {
            _stream,
            sink,
            source,
            now_playing: Cell::new(None),
        })
    }

    pub fn play(&self) -> anyhow::Result<(), MusicPlayerError> {
        let source = self.source.borrow();

        if source.get_songs().len() > 0 && self.now_playing.get() == None {
            self.now_playing.set(Some(0)) // first song of the list
        } else {
            return Err(MusicPlayerError::PlaylistError(String::from(
                "Playlist is empty!",
            )));
        }

        let song = source
            .get_song(self.now_playing.get().unwrap() as usize)
            .ok_or(MusicPlayerError::FileNotFound(String::from(
                "Audio file not found",
            )))?;

        let file = File::open(song.path.clone())?;
        let source = Decoder::new(BufReader::new(file))?;
        self.sink.append(source);
        self.resume();
        Ok(())
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn current_song(&self) -> Option<Song> {
        if let Some(idx) = self.now_playing.get() {
            self.source.borrow().get_song(idx).cloned()
        } else {
            None
        }
    }
}
