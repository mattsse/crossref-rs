use crate::error::Result;
use crate::query::works::{WorksCombiner, WorksFilter, WorksIdentQuery, WorksQuery};
use crate::query::{Component, CrossrefQuery, CrossrefRoute, ResourceComponent};

/// constructs the request payload for the `/journals` route
#[derive(Debug, Clone)]
pub enum Journals {
    /// target a specific journal at `/journals/{id}`
    Identifier(String),
    /// target a `Work` for a specific funder at `/journals/{id}/works?query..`
    Works(WorksIdentQuery),
}

impl CrossrefRoute for Journals {
    fn route(&self) -> Result<String> {
        match self {
            Journals::Identifier(s) => Ok(format!("{}/{}", Component::Journals.route()?, s)),
            Journals::Works(combined) => Self::combined_route(combined),
        }
    }
}

impl CrossrefQuery for Journals {
    fn resource_component(self) -> ResourceComponent {
        ResourceComponent::Journals(self)
    }
}
