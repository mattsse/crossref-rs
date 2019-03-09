use failure::{Backtrace, Compat, Context, Fail};
use serde::{de, ser};
use std::{fmt, result};

/// A type alias for handling errors throughout crossref.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur while interacting with a crossref index.
#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

/// all different error types this crate uses
#[derive(Debug, Clone, Fail, PartialEq)]
pub enum ErrorKind {
    /// if an invalid type was requested
    #[fail(display = "invalid type name: {}", name)]
    InvalidTypeName { name: String },

    /// if a error in serde occurred
    #[fail(display = "invalid serde: {}", msg)]
    InvalidSerde { msg: String },
}

/// helper to make `Error` usable in serde
#[derive(Debug)]
pub struct SerdeErr {
    /// wrapped error
    pub error: Error,
}

impl ser::Error for SerdeErr {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerdeErr {
            error: Error::from(ErrorKind::InvalidSerde {
                msg: msg.to_string(),
            }),
        }
    }
}

impl de::Error for SerdeErr {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerdeErr {
            error: Error::from(ErrorKind::InvalidSerde {
                msg: msg.to_string(),
            }),
        }
    }
}

impl fmt::Display for SerdeErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl std::error::Error for SerdeErr {
    fn description(&self) -> &str {
        "An error has occurred."
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}

impl From<SerdeErr> for Error {
    fn from(serde: SerdeErr) -> Error {
        serde.error
    }
}
