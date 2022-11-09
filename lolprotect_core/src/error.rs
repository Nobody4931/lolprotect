use std::{ffi::OsString, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Standard IO error
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Invalid path string
    #[error("invalid path: {0:?}")]
    InvalidPath(OsString),

    /// Invalid lol file format
    #[error("invalid file: {0}")]
    InvalidFile(PathBuf),

    /// Incorrect decryption password
    #[error("incorrect password")]
    IncorrectPassword,

    /// Corrupted lol file contents
    #[error("corrupted file: {0}")]
    CorruptedFile(PathBuf),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, Error>;
