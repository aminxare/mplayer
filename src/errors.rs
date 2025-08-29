use std::io;

use rodio::{decoder::DecoderError, PlayError, StreamError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MusicPlayerError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Audio decoding error: {0}")]
    AudioDecoderError(#[from] DecoderError),
    #[error("Play error: {0}")]
    PlayError(#[from] PlayError),
    #[error("Stream error: {0}")]
    StreamError(#[from] StreamError),
    #[error("Playlist error: {0}")]
    PlaylistError(String),
    #[error("UI error: {0}")]
    UiError(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
}
