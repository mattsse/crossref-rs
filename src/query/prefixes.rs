use crate::error::Result;
use crate::query::works::{WorksCombiner, WorksFilter, WorksIdentQuery, WorksQuery};
use crate::query::{Component, CrossrefQuery, CrossrefRoute, ResourceComponent};

/// constructs the request payload for the `/prefixes` route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Prefixes {
    /// target a specific member at `/prefixes/{id}`
    Identifier(String),
    /// target a `Work` for a specific prefix at `/prefixes/{id}/works?query..`
    Works(WorksIdentQuery),
}

impl CrossrefRoute for Prefixes {
    fn route(&self) -> Result<String> {
        match self {
            Prefixes::Identifier(s) => Ok(format!("{}/{}", Component::Prefixes.route()?, s)),
            Prefixes::Works(combined) => Self::combined_route(combined),
        }
    }
}

impl CrossrefQuery for Prefixes {
    fn resource_component(self) -> ResourceComponent {
        ResourceComponent::Prefixes(self)
    }
}
