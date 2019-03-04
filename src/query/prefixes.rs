use crate::error::Result;
use crate::query::works::{WorkFilter, WorksCombined, WorksQuery};
use crate::query::{Component, CrossrefRoute};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Prefixes {
    Identifier(String),
    Works(WorksCombined),
}

impl CrossrefRoute for Prefixes {
    fn route(&self) -> Result<String> {
        match self {
            Prefixes::Identifier(s) => Ok(format!("{}/{}", Component::Prefixes.route()?, s)),
            Prefixes::Works(combined) => {
                let query = combined.query.route()?;
                if query.is_empty() {
                    Ok(format!(
                        "{}/{}/{}",
                        Component::Prefixes.route()?,
                        combined.id,
                        Component::Works.route()?
                    ))
                } else {
                    Ok(format!(
                        "{}/{}/{}?{}",
                        Component::Prefixes.route()?,
                        combined.id,
                        Component::Works.route()?,
                        query
                    ))
                }
            }
        }
    }
}
