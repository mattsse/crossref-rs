use crate::error::Result;
use crate::query::facet::FacetCount;
use crate::query::works::{WorksCombiner, WorksFilter, WorksIdentQuery, WorksQuery};
use crate::query::*;
use std::borrow::Cow;

/// filters supported for the /funders route
#[derive(Debug, Clone)]
pub enum FundersFilter {
    /// funders located in specified country
    Location(String),
}

impl FundersFilter {
    /// the key name for the filter element
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

/// constructs the request payload for the `/funders` route
#[derive(Debug, Clone)]
pub enum Funders {
    /// target a specific funder at `/funder/{id}`
    Identifier(String),
    /// target all funders that match the query at `/funders?query...`
    Query(FundersQuery),
    /// target a `Work` for a specific funder at `/funders/{id}/works?query..`
    Works(WorksIdentQuery),
}

impl CrossrefRoute for Funders {
    fn route(&self) -> Result<String> {
        match self {
            Funders::Identifier(s) => Ok(format!("{}/{}", Component::Funders.route()?, s)),
            Funders::Query(query) => {
                let query = query.route()?;
                if query.is_empty() {
                    Component::Funders.route()
                } else {
                    Ok(format!("{}?{}", Component::Funders.route()?, query))
                }
            }
            Funders::Works(combined) => Self::combined_route(combined),
        }
    }
}

impl CrossrefQuery for Funders {
    fn resource_component(self) -> ResourceComponent {
        ResourceComponent::Funders(self)
    }
}
