use crate::error::{Error, Result};
use crate::model::*;
use crate::proto::MessageType::WorkList;
use crate::query::facet::FacetCount;
use crate::query::*;
use crate::serialize::Serializer;
use crate::types::Types;
use chrono::NaiveDate;
use serde::Serialize;
use serde::Serializer as SerdeSerializer;
use serde_json::Value;
use std::borrow::Cow;

/// Filters allow you to narrow queries. All filter results are lists
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkFilter {
    /// metadata which includes one or more funder entry
    HasFunder,
    /// metadata which include the `id` in FundRef data
    Funder(String),
    /// funder records where location = `{country name}`.
    /// Only works on `/funders` route
    Location(String),
    /// metadata belonging to a DOI owner prefix `{owner_prefix}` (e.g. 10.1016 )
    Prefix(String),
    /// metadata belonging to a Crossref member
    Member(String),
    /// metadata indexed since (inclusive)
    FromIndexDate(NaiveDate),
    /// metadata indexed before (inclusive)
    UntilIndexDate(NaiveDate),
    /// metadata last (re)deposited since (inclusive)
    FromDepositDate(NaiveDate),
    /// metadata last (re)deposited before (inclusive)
    UntilDepositDate(NaiveDate),
    /// Metadata updated since (inclusive) {date}.
    /// Currently the same as `from-deposit-date`
    FromUpdateDate(NaiveDate),
    /// Metadata updated before (inclusive) {date}.
    /// Currently the same as `until-deposit-date`
    UntilUpdateDate(NaiveDate),
    /// metadata first deposited since (inclusive)
    FromCreatedDate(NaiveDate),
    /// metadata first deposited before (inclusive)
    UntilCreatedDate(NaiveDate),
    /// metadata where published date is since (inclusive)
    FromPubDate(NaiveDate),
    /// metadata where published date is before (inclusive)
    UntilPubDate(NaiveDate),
    /// metadata where online published date is since (inclusive)
    FromOnlinePubDate(NaiveDate),
    /// metadata where online published date is before (inclusive)
    UntilOnlinePubDate(NaiveDate),
    /// metadata where print published date is since (inclusive)
    FromPrintPubDate(NaiveDate),
    /// metadata where print published date is before (inclusive)
    UntilPrintPubDate(NaiveDate),
    /// metadata where posted date is since (inclusive)
    FromPostedDate(NaiveDate),
    /// metadata where posted date is before (inclusive)
    UntilPostedDate(NaiveDate),
    /// metadata where accepted date is since (inclusive)
    FromAcceptedDate(NaiveDate),
    /// metadata where accepted date is before (inclusive)
    UntilAcceptedDate(NaiveDate),
    /// metadata that includes any `<license_ref>` elements.
    HasLicense,
    /// metadata where `<license_ref> value equals the value
    LicenseUrl(String),
    /// metadata where the `<license_ref>`'s applies_to attribute is
    LicenseVersion(String),
    /// metadata where difference between publication date and the `<license_ref>`'s start_date attribute is <= value (in days)
    LicenseDelay(i32),
    /// metadata that includes any full text `<resource>` elements
    HasFullText,
    /// metadata where `<resource>` element's content_version attribute is the value
    FullTextVersion(String),
    /// metadata where `<resource>` element's content_type attribute is value (e.g. `application/pdf)`
    FullTextType(String),
    /// metadata where `<resource>` link has one of the following intended applications: `text-mining`, `similarity-checking` or `unspecified`
    FullTextApplication(String),
    /// metadata for works that have a list of references
    HasReferences,
    /// metadata for works where references are either `open`, `limited` (to Metadata Plus subscribers) or `closed`
    ReferenceVisibility(Visibility),
    /// metadata which include name of archive partner
    HasArchive,
    ///  metadata which where value of archive partner is the value
    Archive(String),
    /// metadata which includes one or more ORCIDs
    HasOrcid,
    /// metadata which includes one or more ORCIDs where the depositing publisher claims to have witness the ORCID owner authenticate with ORCID
    HasAuthenticatedOrcid,
    /// metadata where `<orcid>` element's value = the value
    Orcid(String),
    /// metadata where record has an ISSN = the value. Format is xxxx-xxxx
    Issn(String),
    /// metadata where record has an ISBN = the value
    Isbn(String),
    /// metadata records whose type = value.
    /// Type must be an ID value from the list of types returned by the `/types` resource
    Type(Types),
    /// metadata records whose article or serial are mentioned in the given value.
    /// Currently the only supported value is `doaj`
    Directory(String),
    /// metadata describing the DOI
    Doi(String),
    /// metadata for records that represent editorial updates to the DOI
    Updates(String),
    /// metadata for records that represent editorial updates
    IsUpdate,
    /// metadata for records that include a link to an editorial update policy
    HasUpdatePolicy,
    /// metadata for records with a publication title exactly with an exact match
    ContainerTitle(String),
    /// metadata for records with an exact matching category label.
    /// Category labels come from [this list](https://www.elsevier.com/solutions/scopus/content) published by Scopus
    CategoryName(String),
    /// metadata for records with an exacty matching type label
    TypeName(String),
    /// metadata for records with a matching award number.
    /// Optionally combine with `award.funder`
    AwardNumber(String),
    /// metadata for records with an award with matching funder.
    /// Optionally combine with `award.number`
    AwardFunder(String),
    /// metadata for records with any assertions
    HasAssertion,
    /// metadata for records with an assertion in a particular group
    AssertionGroup(String),
    /// metadata for records with a particular named assertion
    Assertion(String),
    /// metadata for records that have any affiliation information
    HasAffiliation,
    /// metadata for records with the given alternative ID,
    /// which may be a publisher-specific ID, or any other identifier a publisher may have provided
    AlternativeId,
    /// metadata for records with a given article number
    ArticleNumber,
    /// metadata for records which include an abstract
    HasAbstract,
    /// metadata for records which include a clinical trial number
    HasClinicalTrialNumber,
    /// metadata where the publisher records a particular domain name as the location Crossmark content will appear
    ContentDomain(String),
    /// metadata where the publisher records a domain name location for Crossmark content
    HasContentDomain,
    /// metadata where the publisher restricts Crossmark usage to content domains
    HasDomainRestriction,
    /// metadata for records that either assert or are the object of a relation
    HasRelation,
    /// One of the relation types from the Crossref relations schema
    /// (e.g. `is-referenced-by`, `is-parent-of`, `is-preprint-of`)
    RelationType,
    /// Relations where the object identifier matches the identifier provided
    RelationObject,
    /// One of the identifier types from the Crossref relations schema (e.g. `doi`, `issn`)
    RelationObjectType(String),
}

impl WorkFilter {
    pub fn name(&self) -> &str {
        match self {
            WorkFilter::HasFunder => "has-funder",
            WorkFilter::Funder(_) => "funder",
            WorkFilter::Location(_) => "location",
            WorkFilter::Prefix(_) => "prefix",
            WorkFilter::Member(_) => "member",
            WorkFilter::FromIndexDate(_) => "from-index-date",
            WorkFilter::UntilIndexDate(_) => "until-index-date",
            WorkFilter::FromDepositDate(_) => "from-deposit-date",
            WorkFilter::UntilDepositDate(_) => "until-deposit-date",
            WorkFilter::FromUpdateDate(_) => "from-update-date",
            WorkFilter::UntilUpdateDate(_) => "until-update-date",
            WorkFilter::FromCreatedDate(_) => "from-created-date",
            WorkFilter::UntilCreatedDate(_) => "until-created-date",
            WorkFilter::FromPubDate(_) => "from-pub-date",
            WorkFilter::UntilPubDate(_) => "until-pub-date",
            WorkFilter::FromOnlinePubDate(_) => "from-online-pub-date",
            WorkFilter::UntilOnlinePubDate(_) => "until-online-pub-date",
            WorkFilter::FromPrintPubDate(_) => "from-print-pub-date",
            WorkFilter::UntilPrintPubDate(_) => "until-print-pub-date",
            WorkFilter::FromPostedDate(_) => "from-posted-date",
            WorkFilter::UntilPostedDate(_) => "until-posted-date",
            WorkFilter::FromAcceptedDate(_) => "from-accepted-date",
            WorkFilter::UntilAcceptedDate(_) => "until-accepted-date",
            WorkFilter::HasLicense => "has-license",
            WorkFilter::LicenseUrl(_) => "license.url",
            WorkFilter::LicenseVersion(_) => "license.version",
            WorkFilter::LicenseDelay(_) => "license.delay",
            WorkFilter::HasFullText => "has-full-text",
            WorkFilter::FullTextVersion(_) => "full-text.version",
            WorkFilter::FullTextType(_) => "full-text.type",
            WorkFilter::FullTextApplication(_) => "full-text.application",
            WorkFilter::HasReferences => "has-references",
            WorkFilter::ReferenceVisibility(_) => "reference-visibility",
            WorkFilter::HasArchive => "has-archive",
            WorkFilter::Archive(_) => "archive",
            WorkFilter::HasOrcid => "has-orcid",
            WorkFilter::HasAuthenticatedOrcid => "has-authenticated-orcid",
            WorkFilter::Orcid(_) => "orcid",
            WorkFilter::Issn(_) => "issn",
            WorkFilter::Isbn(_) => "isbn",
            WorkFilter::Type(_) => "type",
            WorkFilter::Directory(_) => "directory",
            WorkFilter::Doi(_) => "doi",
            WorkFilter::Updates(_) => "updates",
            WorkFilter::IsUpdate => "is-update",
            WorkFilter::HasUpdatePolicy => "has-update-policy",
            WorkFilter::ContainerTitle(_) => "container-title",
            WorkFilter::CategoryName(_) => "category-name",
            WorkFilter::TypeName(_) => "type-name",
            WorkFilter::AwardNumber(_) => "award.number",
            WorkFilter::AwardFunder(_) => "award.funder",
            WorkFilter::HasAssertion => "has-assertion",
            WorkFilter::AssertionGroup(_) => "assertion-group",
            WorkFilter::Assertion(_) => "assertion",
            WorkFilter::HasAffiliation => "has-affiliation",
            WorkFilter::AlternativeId => "alternative-id",
            WorkFilter::ArticleNumber => "article-number",
            WorkFilter::HasAbstract => "has-abstract",
            WorkFilter::HasClinicalTrialNumber => "has-clinical-trial-number	",
            WorkFilter::ContentDomain(_) => "content-domain",
            WorkFilter::HasContentDomain => "has-content-domain",
            WorkFilter::HasDomainRestriction => "has-domain-restriction",
            WorkFilter::HasRelation => "has-relation",
            WorkFilter::RelationType => "relation.type",
            WorkFilter::RelationObject => "relation.object",
            WorkFilter::RelationObjectType(_) => "relation.object-type",
        }
    }
}

impl ParamFragment for WorkFilter {
    fn key(&self) -> Cow<str> {
        Cow::Borrowed(self.name())
    }

    fn value(&self) -> Option<Cow<str>> {
        match self {
            WorkFilter::Funder(s)
            | WorkFilter::Location(s)
            | WorkFilter::Prefix(s)
            | WorkFilter::Member(s)
            | WorkFilter::LicenseUrl(s)
            | WorkFilter::LicenseVersion(s)
            | WorkFilter::FullTextVersion(s)
            | WorkFilter::FullTextType(s)
            | WorkFilter::FullTextApplication(s)
            | WorkFilter::Archive(s)
            | WorkFilter::Orcid(s)
            | WorkFilter::Issn(s)
            | WorkFilter::Isbn(s)
            | WorkFilter::Directory(s)
            | WorkFilter::Doi(s)
            | WorkFilter::Updates(s)
            | WorkFilter::ContainerTitle(s)
            | WorkFilter::CategoryName(s)
            | WorkFilter::AwardNumber(s)
            | WorkFilter::TypeName(s)
            | WorkFilter::AwardFunder(s)
            | WorkFilter::AssertionGroup(s)
            | WorkFilter::Assertion(s)
            | WorkFilter::ContentDomain(s)
            | WorkFilter::RelationObjectType(s) => Some(Cow::Borrowed(s.as_str())),
            WorkFilter::ReferenceVisibility(vis) => Some(Cow::Borrowed(vis.as_str())),
            WorkFilter::FromIndexDate(d)
            | WorkFilter::UntilIndexDate(d)
            | WorkFilter::FromDepositDate(d)
            | WorkFilter::UntilDepositDate(d)
            | WorkFilter::FromUpdateDate(d)
            | WorkFilter::UntilUpdateDate(d)
            | WorkFilter::FromCreatedDate(d)
            | WorkFilter::UntilCreatedDate(d)
            | WorkFilter::FromPubDate(d)
            | WorkFilter::UntilPubDate(d)
            | WorkFilter::FromOnlinePubDate(d)
            | WorkFilter::UntilOnlinePubDate(d)
            | WorkFilter::FromPrintPubDate(d)
            | WorkFilter::UntilPrintPubDate(d)
            | WorkFilter::FromPostedDate(d)
            | WorkFilter::UntilPostedDate(d)
            | WorkFilter::FromAcceptedDate(d)
            | WorkFilter::UntilAcceptedDate(d) => {
                Some(Cow::Owned(d.format("%Y-%m-%d").to_string()))
            }
            WorkFilter::Type(t) => Some(Cow::Borrowed(t.id())),
            _ => Some(Cow::Borrowed("true")),
        }
    }
}

impl Filter for WorkFilter {}

/// Field queries are available on the `/works` route and allow for queries that match only particular fields of metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldQuery {
    /// match any only particular fields of metadata.
    pub name: String,
    /// the value of the query
    pub value: String,
}

impl FieldQuery {
    /// creates a new [Field] query for `title` and `subtitle`
    pub fn title(title: &str) -> Self {
        Self {
            name: "title".to_string(),
            value: title.to_string(),
        }
    }

    /// creates a new [Field] query for `container-title` aka `publication.name`
    pub fn container_title(container_title: &str) -> Self {
        Self {
            name: "container-title".to_string(),
            value: container_title.to_string(),
        }
    }
    /// creates a new [Field] query author given and family names
    pub fn author(author: &str) -> Self {
        Self {
            name: "author".to_string(),
            value: author.to_string(),
        }
    }
    /// creates a new [Field] query for editor given and family names
    pub fn editor(editor: &str) -> Self {
        Self {
            name: "editor".to_string(),
            value: editor.to_string(),
        }
    }
    /// creates a new [Field] query for chair given and family names
    pub fn chair(chair: &str) -> Self {
        Self {
            name: "chair".to_string(),
            value: chair.to_string(),
        }
    }
    /// creates a new [Field] query for translator given and family names
    pub fn translator(translator: &str) -> Self {
        Self {
            name: "translator".to_string(),
            value: translator.to_string(),
        }
    }
    /// creates a new [Field] query for author, editor, chair and translator given and family names
    pub fn contributor(contributor: &str) -> Self {
        Self {
            name: "contributor".to_string(),
            value: contributor.to_string(),
        }
    }
    /// creates a new [Field] query for bibliographic information, useful for citation look up.
    /// Includes titles, authors, ISSNs and publication years
    pub fn bibliographic(bibliographic: &str) -> Self {
        Self {
            name: "bibliographic".to_string(),
            value: bibliographic.to_string(),
        }
    }
    /// creates a new [Field] query for contributor affiliations
    pub fn affiliation(affiliation: &str) -> Self {
        Self {
            name: "affiliation".to_string(),
            value: affiliation.to_string(),
        }
    }
}

impl CrossrefQueryParam for FieldQuery {
    fn param_key(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }
    fn param_value(&self) -> Option<Cow<str>> {
        Some(Cow::Owned(format_query(&self.value)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkResultControl {
    /// use the standard ResultControl available for all components
    Standard(ResultControl),
    /// If you are expecting results beyond 10K, then use a cursor to deep page through the results
    Cursor {
        /// the cursor token provided by crossref when initially set to a value of `*`
        token: Option<String>,
        /// limit the results
        rows: Option<usize>,
    },
}

impl WorkResultControl {
    /// set a cursor with `*` value, a new cursor will be provided in the `next-cursor` field of the result
    pub fn new_cursor() -> Self {
        WorkResultControl::Cursor {
            token: None,
            rows: None,
        }
    }

    /// create a new Cursor with only a token value
    pub fn cursor(token: &str) -> Self {
        WorkResultControl::Cursor {
            token: Some(token.to_string()),
            rows: None,
        }
    }
}

impl CrossrefQueryParam for WorkResultControl {
    fn param_key(&self) -> Cow<str> {
        match self {
            WorkResultControl::Standard(s) => s.param_key(),
            WorkResultControl::Cursor { token, .. } => Cow::Owned(format!(
                "cursor={}",
                token.as_ref().map(|x| x.as_str()).unwrap_or("*")
            )),
        }
    }

    fn param_value(&self) -> Option<Cow<str>> {
        match self {
            WorkResultControl::Standard(s) => s.param_value(),
            WorkResultControl::Cursor { rows, .. } => match rows {
                Some(r) => Some(Cow::Owned(format!("rows={}", r))),
                _ => None,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Works {
    /// target a Work by a specific id
    Identifier(String),
    /// target Works by a query
    Query(WorksQuery),
    /// return the registration agency for a DOI
    Agency(String),
}

impl Works {
    /// create a new `Works::Identifier` by converting `id` to a `String`
    fn new_id(doi: &str) -> Self {
        Works::Identifier(doi.to_string())
    }
    /// create a new `Works::Agency` targeting the registration agency for the DOI
    fn agency_for_doi(doi: &str) -> Self {
        Works::Agency(doi.to_string())
    }
}

impl CrossrefRoute for Works {
    fn route(&self) -> Result<String> {
        match self {
            Works::Identifier(s) => Ok(format!("{}/{}", Component::Works.route()?, s)),
            Works::Agency(s) => Ok(format!("{}/{}/agency", Component::Works.route()?, s)),
            Works::Query(query) => {
                let query = query.route()?;
                if query.is_empty() {
                    Component::Works.route()
                } else {
                    Ok(format!("{}?{}", Component::Works.route()?, query))
                }
            }
        }
    }
}

impl CrossrefQuery for Works {
    fn resource_component(&self) -> ResourceComponent {
        ResourceComponent::Single(Component::Works)
    }
}

///
/// Each query parameter is ANDed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorksQuery {
    /// search by non specific query
    pub free_form_queries: Vec<String>,
    /// match only particular fields of metadata
    pub field_queries: Vec<FieldQuery>,
    /// filter to apply while querying
    pub filter: Vec<WorkFilter>,
    /// sort results by a certain field and
    pub sort: Option<Sort>,
    /// set the sort order to `asc` or `desc`
    pub order: Option<Order>,
    /// enable facet information in responses
    pub facets: Vec<FacetCount>,
    /// deep page through `/works` result sets
    pub result_control: Option<WorkResultControl>,
}

impl WorksQuery {
    /// add a new free form query
    pub fn query(mut self, query: &str) -> Self {
        self.free_form_queries.push(query.to_string());
        self
    }
    /// add a new field query form query
    pub fn field_query(mut self, query: FieldQuery) -> Self {
        self.field_queries.push(query);
        self
    }
    /// add a new filter to the query
    pub fn filter(mut self, filter: WorkFilter) -> Self {
        self.filter.push(filter);
        self
    }

    /// set sort option to the query
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    /// set order option to query
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    /// add another facet to query
    pub fn facet(mut self, facet: FacetCount) -> Self {
        self.facets.push(facet);
        self
    }

    /// set result control option to query
    pub fn result_control(mut self, result_control: WorkResultControl) -> Self {
        self.result_control = Some(result_control);
        self
    }
}

impl CrossrefRoute for WorksQuery {
    fn route(&self) -> Result<String> {
        let mut params = Vec::new();
        if !self.free_form_queries.is_empty() {
            params.push(Cow::Owned(format!(
                "query={}",
                format_queries(&self.free_form_queries)
            )));
        }
        if !self.field_queries.is_empty() {
            params.extend(self.field_queries.iter().map(|x| x.param()))
        }
        if !self.filter.is_empty() {
            params.push(self.filter.param());
        }
        if !self.facets.is_empty() {
            params.push(self.facets.param());
        }
        if let Some(sort) = &self.sort {
            params.push(sort.param());
        }
        if let Some(order) = &self.order {
            params.push(order.param());
        }
        if let Some(rc) = &self.result_control {
            params.push(rc.param());
        }

        Ok(params.join("&"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize::to_string;

    #[test]
    fn serialize_works_filter() {
        let filter = WorkFilter::FromAcceptedDate(NaiveDate::from_ymd(2019, 10, 10));
        assert_eq!(
            "from-accepted-date=2019-10-10",
            &to_string(&filter).unwrap()
        );
    }

    #[test]
    fn serialize_works_ident() {
        let works = Works::new_id("10.1037/0003-066X.59.1.29");

        assert_eq!("/works/10.1037/0003-066X.59.1.29", &works.route().unwrap())
    }

}
