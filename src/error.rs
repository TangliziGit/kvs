use failure::Fail;
use std::io;

/// Error type for kvs.
#[derive(Debug, Fail)]
pub enum Error {
    /// IO error.
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Serialization or deserialization error.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    /// Error for key not found.
    #[fail(display = "Key not found")]
    KeyNotFound,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err)
    }
}

/// Defined KvStore Result include a KvStore Error.
pub type Result<T> = std::result::Result<T, Error>;
