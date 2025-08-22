use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct AudioPlayer {
    _stream: OutputStream, // نگه داشتن OutputStream برای جلوگیری از drop شدن
    sink: Sink,
}

impl AudioPlayer {
    pub fn new() -> anyhow::Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        Ok(AudioPlayer { _stream, sink })
    }

    pub fn play(&self, path: &PathBuf) -> anyhow::Result<()> {
        let file = File::open(path)?;
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
}