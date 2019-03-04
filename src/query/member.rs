use crate::error::Result;
use crate::query::works::{WorkFilter, WorksCombined, WorksQuery};
use crate::query::*;
use std::borrow::Cow;

/// filters supported for the `/members` route
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MembersFilter {
    /// Member has made their references public for one or more of their prefixes
    HasPublicReferences,
    /// metadata for works where references are either `open`, `limited` (to Metadata Plus subscribers) or `closed`
    ReferenceVisibility(Visibility),
    /// count of DOIs for material published more than two years ago
    BlackfileDoiCount(i32),
    /// count of DOIs for material published within last two years
    CurrentDoiCount(i32),
}

impl MembersFilter {
    pub fn name(&self) -> &str {
        match self {
            MembersFilter::HasPublicReferences => "has-public-references",
            MembersFilter::ReferenceVisibility(_) => "reference-visibility",
            MembersFilter::BlackfileDoiCount(_) => "blackfile-doi-count",
            MembersFilter::CurrentDoiCount(_) => "current-doi-count",
        }
    }
}

impl ParamFragment for MembersFilter {
    fn key(&self) -> Cow<str> {
        Cow::Borrowed(self.name())
    }

    fn value(&self) -> Option<Cow<str>> {
        match self {
            MembersFilter::HasPublicReferences => None,
            MembersFilter::ReferenceVisibility(vis) => Some(Cow::Borrowed(vis.as_str())),
            MembersFilter::BlackfileDoiCount(num) => Some(Cow::Owned(num.to_string())),
            MembersFilter::CurrentDoiCount(num) => Some(Cow::Owned(num.to_string())),
        }
    }
}

impl Filter for MembersFilter {}

impl_common_query!(MembersQuery, MembersFilter);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Members {
    Identifier(String),
    Query(MembersQuery),
    Works(WorksCombined),
}

impl CrossrefRoute for Members {
    fn route(&self) -> Result<String> {
        match self {
            Members::Identifier(s) => Ok(format!("{}/{}", Component::Members.route()?, s)),
            Members::Query(query) => {
                let query = query.route()?;
                if query.is_empty() {
                    Component::Members.route()
                } else {
                    Ok(format!("{}?{}", Component::Members.route()?, query))
                }
            }
            Members::Works(combined) => {
                let query = combined.query.route()?;
                if query.is_empty() {
                    Ok(format!(
                        "{}/{}/{}",
                        Component::Members.route()?,
                        combined.id,
                        Component::Works.route()?
                    ))
                } else {
                    Ok(format!(
                        "{}/{}/{}?{}",
                        Component::Members.route()?,
                        combined.id,
                        Component::Works.route()?,
                        query
                    ))
                }
            }
        }
    }
}
