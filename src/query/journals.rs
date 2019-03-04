use crate::error::Result;
use crate::query::works::{WorkFilter, WorksCombined, WorksQuery};
use crate::query::{Component, CrossrefRoute};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Journals {
    Identifier(String),
    Works(WorksCombined),
}

impl CrossrefRoute for Journals {
    fn route(&self) -> Result<String> {
        match self {
            Journals::Identifier(s) => Ok(format!("{}/{}", Component::Journals.route()?, s)),
            Journals::Works(combined) => {
                let query = combined.query.route()?;
                if query.is_empty() {
                    Ok(format!(
                        "{}/{}/{}",
                        Component::Journals.route()?,
                        combined.id,
                        Component::Works.route()?
                    ))
                } else {
                    Ok(format!(
                        "{}/{}/{}?{}",
                        Component::Journals.route()?,
                        combined.id,
                        Component::Works.route()?,
                        query
                    ))
                }
            }
        }
    }
}
