use std::result;

/// A type alias for handling errors throughout crossref.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur while interacting with a crossref index.
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid type name: {}", name)]
    InvalidTypeName { name: String },
}
