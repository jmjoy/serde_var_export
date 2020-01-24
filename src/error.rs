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

impl Error {
    fn to_message(&self) -> Option<&str> {
        if let Error::Message(msg) = &self {
            Some(msg)
        } else {
            None
        }
    }
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

#[cfg(test)]
mod tests {
    use super::Error;
    use serde::{de, ser};

    #[test]
    fn ser_error() {
        let err: Error = ser::Error::custom("some message");
        assert_eq!(err.to_message(), Some("some message"));
    }

    #[test]
    fn de_error() {
        let err: Error = de::Error::custom("some message");
        assert_eq!(err.to_message(), Some("some message"));
    }
}
