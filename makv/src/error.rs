use sled;
use std::io;
use thiserror::Error;

/// Represent all Yakv error
#[derive(Error, Debug)]
#[error("...")]
pub enum MakvError {
    /// Any Error
    Any(#[from] anyhow::Error),

    /// IO Error
    Io(#[from] io::Error),

    /// Serde Error
    Serde(#[from] serde_json::Error),

    /// Sled Error
    Sled(#[from] sled::Error),

    /// Unexpected Command Error
    #[error("Unexpected command")]
    UnexpectedCommand,

    /// Not found error
    #[error("Key not found: {0}")]
    NotFoundError(String),
}

/// Result handles Result<T, YakvError>
pub type Result<T> = anyhow::Result<T, MakvError>;
