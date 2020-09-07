use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::io;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "Key not found")]
    KeyNotFound,

    #[fail(display = "Value not found")]
    ValueNotFound,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error {
            inner: Context::new(ErrorKind::Io(err)),
        }
    }
}

/// Defined KvStore Result include a KvStore Error.
pub type Result<T> = std::result::Result<T, Error>;
