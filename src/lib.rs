//#![deny(warnings)]
//#![deny(missing_docs)]
#![allow(unused)]
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

mod error;
mod model;
mod proto;
mod query;
mod serialize;
mod types;

#[cfg(feature = "client")]
mod client;

// https://github.com/sckott/habanero/blob/master/habanero/crossref/crossref.py

// either one general query method and typed query objects or for each target individually

/// Struct for Crossref search API methods
#[derive(Debug, Clone, Default)]
struct Crossref {
    pub mailto: Option<String>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
}

impl Crossref {}
