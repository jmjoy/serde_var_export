use serde::{de, ser};
use std::fmt::Display;
use std::io;
use std::string::FromUtf8Error;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error("io error: {0:?}")]
    Io(#[from] io::Error),

    #[error("from utf8 error: {0:?}")]
    FromUtf8(#[from] FromUtf8Error),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
