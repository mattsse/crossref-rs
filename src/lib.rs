//#![deny(warnings)]
//#![deny(missing_docs)]
#![allow(unused)]
#[macro_use]
extern crate serde_derive;

use crate::model::ResourceLink;

mod error;
pub mod model;
pub mod proto;
pub mod query;

#[cfg(feature = "client")]
pub mod client;

#[doc(inline)]
pub use self::error::{Error, Result};

#[doc(inline)]
pub use self::query::works::{
    FieldQuery, WorkFilter, WorkResultControl, Works, WorksCombined, WorksQuery,
};
#[doc(inline)]
pub use self::query::{CrossrefRoute, Order, Sort};

/// A convenience module appropriate for glob imports (`use crossref::prelude::*;`).
use serde::{Deserialize, Serialize};

// https://github.com/sckott/habanero/blob/master/habanero/crossref/crossref.py

// either one general query method and typed query objects or for each target individually

/// Struct for Crossref search API methods
#[derive(Debug, Clone, Default)]
struct Crossref {
    /// will be included in the `mailto` parameter of the request query
    /// so that crossref can contact you if your script misbehaves.
    /// this will get you directed to the "polite pool"
    pub mailto: Option<String>,
    /// use another base url than `api.crossref.org`
    pub base_url: Option<String>,
    /// set an api key if available
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
