use crate::error::Result;
use crate::query::facet::FacetCount;
use crate::query::works::WorkFilter;
use crate::query::*;
use std::borrow::Cow;

/// filters supported for the /funders route
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FundersFilter {
    /// funders located in specified country
    Location(String),
}

impl FundersFilter {
    pub fn name(&self) -> &str {
        match self {
            FundersFilter::Location(_) => "location",
        }
    }
}

impl ParamFragment for FundersFilter {
    fn key(&self) -> Cow<str> {
        Cow::Borrowed(self.name())
    }

    fn value(&self) -> Option<Cow<str>> {
        match self {
            FundersFilter::Location(s) => Some(Cow::Borrowed(s.as_str())),
        }
    }
}

impl Filter for FundersFilter {}

impl_common_query!(FundersQuery, FundersFilter);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Funders {
    Identifier(String),
    Query(FundersFilter),
    Works { id: String, work: WorkFilter },
}
