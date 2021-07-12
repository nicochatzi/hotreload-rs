use std::{io, path::PathBuf, time::SystemTimeError};

use thiserror::Error;

pub type HotreloadResult<T> = Result<T, HotreloadError>;

#[derive(Error, Debug)]
pub enum HotreloadError {
    #[error("File not found: {0}")]
    FileName(String),

    #[error(transparent)]
    FileOrDir(#[from] io::Error),

    #[error("Invalid path `{0}`")]
    InvalidPath(PathBuf),

    #[error("Time went backwards")]
    TimeRevered(#[from] SystemTimeError),

    #[error(transparent)]
    Hotwatch(#[from] hotwatch::Error),
}
