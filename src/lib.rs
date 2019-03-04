//#![deny(warnings)]
//#![deny(missing_docs)]
#![allow(unused)]
#[macro_use]
extern crate serde_derive;

use crate::model::ResourceLink;

mod error;
mod model;
mod proto;
mod query;

#[cfg(feature = "client")]
mod client;

use crate::error::Result;
use serde::{Deserialize, Serialize};

// https://github.com/sckott/habanero/blob/master/habanero/crossref/crossref.py

// either one general query method and typed query objects or for each target individually

/// Struct for Crossref search API methods
#[derive(Debug, Clone, Default)]
struct Crossref {
    pub mailto: Option<String>,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
}

impl Crossref {
    /// use HTTP HEAD requests to quickly determine "existence" of a singleton.
    fn exists() {
        unimplemented!()
    }

    /// execute the API call and deserialize the message into T
    fn fetch_into<'de, T>() -> Result<T>
    where
        T: Deserialize<'de>,
    {
        unimplemented!()
    }
    /// execute the API call and deserialize the message into T
    fn select<'de, T>() -> Result<T>
    where
        T: Serialize + Deserialize<'de>,
    {
        unimplemented!()
    }
}
