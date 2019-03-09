use crate::query::ResourceComponent;
use crate::response::MessageType;
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
#[derive(Debug, Fail)]
pub enum ErrorKind {
    /// if an invalid type was requested
    #[fail(display = "invalid type name: {}", name)]
    InvalidTypeName { name: String },

    /// if there is a mismatch between the expected return type of the crossref api and this rust client
    #[fail(
        display = "expected response item of type {} but got {}",
        expected, got
    )]
    UnexpectedItem {
        expected: MessageType,
        got: MessageType,
    },
    /// a config error
    #[fail(display = "{}", msg)]
    Config {
        /// the notification
        msg: String,
    },

    /// an error that occurred while operating with [reqwest]
    #[fail(display = "{}", reqwest)]
    ReqWest {
        /// the notification
        reqwest: reqwest::Error,
    },
    /// When no message was found but expected
    #[fail(
        display = "No message found but expected message of type `{}`",
        expected
    )]
    MissingMessage { expected: MessageType },
    /// When crossref could not find anything
    #[fail(display = "Nothing was found for resource `{}`", resource)]
    ResourceNotFound { resource: ResourceComponent },
    /// if a error in serde occurred
    #[fail(display = "invalid serde: {}", error)]
    Serde { error: serde_json::Error },
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

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        ErrorKind::Serde { error }.into()
    }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest: reqwest::Error) -> Error {
        ErrorKind::ReqWest { reqwest }.into()
    }
}
