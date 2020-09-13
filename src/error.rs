use failure::_core::fmt::Formatter;
use failure::{Context, Fail};
use std::fmt::{self, Display};
use std::io;
use std::string::FromUtf8Error;

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

    /// Sled error.
    #[fail(display = "{}", _0)]
    Sled(#[cause] sled::Error),

    /// Error for key not found.
    #[fail(display = "Key not found")]
    KeyNotFound,

    /// Error for bytes-String conversion.
    #[fail(display = "Can not convert bytes into string")]
    FromUtf8Error,

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

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Self {
        Error {
            inner: Context::new(ErrorKind::Sled(err)),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Error {
            inner: Context::new(ErrorKind::FromUtf8Error),
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
