use crate::error::Result;
use crate::query::facet::FacetCount;
use crate::query::works::{WorkFilter, WorksCombined, WorksQuery};
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
    Query(FundersQuery),
    Works(WorksCombined),
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
            Funders::Works(combined) => {
                let query = combined.query.route()?;
                if query.is_empty() {
                    Ok(format!(
                        "{}/{}/{}",
                        Component::Funders.route()?,
                        combined.id,
                        Component::Works.route()?
                    ))
                } else {
                    Ok(format!(
                        "{}/{}/{}?{}",
                        Component::Funders.route()?,
                        combined.id,
                        Component::Works.route()?,
                        query
                    ))
                }
            }
        }
    }
}
