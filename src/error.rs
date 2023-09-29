use std::error::Error;
use std::fmt::Display;

pub type RimResult<T> = Result<T, RimError>;

#[derive(Debug)]
pub enum RimError {
    InvalidMagic,
    IoError(std::io::Error),
}

impl Display for RimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMagic => write!(f, "Invalid magic bytes at start of file"),
            Self::IoError(e) => e.fmt(f),
        }
    }
}

impl Error for RimError {}

impl From<std::io::Error> for RimError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}
