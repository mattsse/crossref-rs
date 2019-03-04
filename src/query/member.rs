use crate::query::works::WorkFilter;
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
    Query(MembersFilter),
    Works { id: String, work: WorkFilter },
}
