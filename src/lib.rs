//#![deny(warnings)]
//#![deny(missing_docs)]
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

mod error;
mod model;
mod types;

#[cfg(feature = "client")]
mod client;

#[derive(Debug, Clone)]
struct Crossref {}
