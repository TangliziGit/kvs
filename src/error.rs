use failure::_core::fmt::Formatter;
use failure::{Context, Fail};
use std::fmt::{self, Display};
use std::io;

/// Error traceable for kvs.
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

/// Error type for kvs.
#[derive(Debug, Fail)]
pub enum ErrorKind {
    /// IO error.
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Serialization or deserialization error.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    /// Error for key not found.
    #[fail(display = "Key not found")]
    KeyNotFound,

    /// Error a KvsClient may receives containing a string.
    #[fail(display = "{}", _0)]
    StringError(String),

    /// Error for unexpected status.
    #[fail(display = "Unexpected: {}", _0)]
    UnexpectedError(&'static str),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error {
            inner: Context::new(ErrorKind::Io(err)),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error {
            inner: Context::new(ErrorKind::Serde(err)),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(err: ErrorKind) -> Self {
        Error {
            inner: Context::new(err),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

/// Defined KvStore Result include a KvStore Error.
pub type Result<T> = std::result::Result<T, Error>;
