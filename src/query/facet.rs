use crate::query::{CrossrefQueryParam, ParamFragment};
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Facet {
    /// Author affiliation
    Affiliation,
    /// Funder literal name as deposited alongside DOIs
    FunderName,
    /// Funder DOI
    FunderDoi,
    /// Contributor ORCID
    ORCID,
    /// Work container title, such as journal title, or book title
    ContainerTitle,
    /// Custom Crossmark assertion name
    Assertion,
    /// Archive location
    Archive,
    /// Significant update type
    UpdateType,
    /// Journal ISSN (any - print, electronic, link)
    ISSN,
    /// Earliest year of publication
    Published,
    /// Work type name, such as `journal-article` or `book-chapter`
    TypeName,
    /// License URI of work
    License,
    /// Category name of work
    CategoryName,
    /// Relation type described by work or described by another work with work as object
    RelationType,
    /// Custom Crossmark assertion group name
    AssertionGroup,
    /// Publisher name of work
    PublisherName,
}

impl Facet {
    /// the maximum numeric number some facets are not allowed to exceed
    pub const MAX_VAL_NUM: usize = 100;

    /// the maximum value the facets can hold
    /// some facets are unbounded `*`, some are limited to [MAX_VAL_NUM]
    fn max_value(&self) -> &str {
        match self {
            Facet::ORCID | Facet::ContainerTitle | Facet::ISSN => "100",
            _ => "*",
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Facet::Affiliation => "affiliation",
            Facet::FunderName => "funder-name",
            Facet::FunderDoi => "funder-doi",
            Facet::ORCID => "orcid",
            Facet::ContainerTitle => "container-title",
            Facet::Assertion => "assertion",
            Facet::Archive => "archive",
            Facet::UpdateType => "update-type",
            Facet::ISSN => "issn",
            Facet::Published => "published",
            Facet::TypeName => "type-name",
            Facet::License => "license",
            Facet::CategoryName => "category-name",
            Facet::RelationType => "relation-type",
            Facet::AssertionGroup => "assertion-group",
            Facet::PublisherName => "publisher-name",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FacetCount {
    /// the targeted facet
    pub facet: Facet,
    /// the maximum number
    pub count: Option<usize>,
}

impl FacetCount {
    fn value(&self) -> String {
        match &self.count {
            Some(count) => match self.facet {
                Facet::ORCID | Facet::ContainerTitle | Facet::ISSN => {
                    if *count > Facet::MAX_VAL_NUM {
                        Facet::MAX_VAL_NUM.to_string()
                    } else {
                        count.to_string()
                    }
                }
                _ => count.to_string(),
            },
            _ => self.facet.max_value().to_string(),
        }
    }
}

impl ParamFragment for FacetCount {
    fn key(&self) -> Cow<str> {
        Cow::Borrowed(self.facet.as_str())
    }
    fn value(&self) -> Option<Cow<str>> {
        Some(Cow::Owned(self.value()))
    }
}

impl CrossrefQueryParam for Vec<FacetCount> {
    fn param_key(&self) -> Cow<str> {
        Cow::Borrowed("facet")
    }

    fn param_value(&self) -> Option<Cow<str>> {
        Some(Cow::Owned(
            self.iter()
                .map(ParamFragment::fragment)
                .collect::<Vec<_>>()
                .join(","),
        ))
    }
}
