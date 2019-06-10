use crate::error::{Error, Result};
use crate::query::facet::FacetCount;
use crate::query::types::Type;
use crate::query::*;
use chrono::NaiveDate;
use serde::Serialize;
use serde::Serializer as SerdeSerializer;
use serde_json::Value;
use std::borrow::Cow;

/// Filters allow you to narrow queries. All filter results are lists
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorksFilter {
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
    Type(Type),
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

impl WorksFilter {
    /// the identifier for a the query key
    pub fn name(&self) -> &str {
        match self {
            WorksFilter::HasFunder => "has-funder",
            WorksFilter::Funder(_) => "funder",
            WorksFilter::Location(_) => "location",
            WorksFilter::Prefix(_) => "prefix",
            WorksFilter::Member(_) => "member",
            WorksFilter::FromIndexDate(_) => "from-index-date",
            WorksFilter::UntilIndexDate(_) => "until-index-date",
            WorksFilter::FromDepositDate(_) => "from-deposit-date",
            WorksFilter::UntilDepositDate(_) => "until-deposit-date",
            WorksFilter::FromUpdateDate(_) => "from-update-date",
            WorksFilter::UntilUpdateDate(_) => "until-update-date",
            WorksFilter::FromCreatedDate(_) => "from-created-date",
            WorksFilter::UntilCreatedDate(_) => "until-created-date",
            WorksFilter::FromPubDate(_) => "from-pub-date",
            WorksFilter::UntilPubDate(_) => "until-pub-date",
            WorksFilter::FromOnlinePubDate(_) => "from-online-pub-date",
            WorksFilter::UntilOnlinePubDate(_) => "until-online-pub-date",
            WorksFilter::FromPrintPubDate(_) => "from-print-pub-date",
            WorksFilter::UntilPrintPubDate(_) => "until-print-pub-date",
            WorksFilter::FromPostedDate(_) => "from-posted-date",
            WorksFilter::UntilPostedDate(_) => "until-posted-date",
            WorksFilter::FromAcceptedDate(_) => "from-accepted-date",
            WorksFilter::UntilAcceptedDate(_) => "until-accepted-date",
            WorksFilter::HasLicense => "has-license",
            WorksFilter::LicenseUrl(_) => "license.url",
            WorksFilter::LicenseVersion(_) => "license.version",
            WorksFilter::LicenseDelay(_) => "license.delay",
            WorksFilter::HasFullText => "has-full-text",
            WorksFilter::FullTextVersion(_) => "full-text.version",
            WorksFilter::FullTextType(_) => "full-text.type",
            WorksFilter::FullTextApplication(_) => "full-text.application",
            WorksFilter::HasReferences => "has-references",
            WorksFilter::ReferenceVisibility(_) => "reference-visibility",
            WorksFilter::HasArchive => "has-archive",
            WorksFilter::Archive(_) => "archive",
            WorksFilter::HasOrcid => "has-orcid",
            WorksFilter::HasAuthenticatedOrcid => "has-authenticated-orcid",
            WorksFilter::Orcid(_) => "orcid",
            WorksFilter::Issn(_) => "issn",
            WorksFilter::Isbn(_) => "isbn",
            WorksFilter::Type(_) => "type",
            WorksFilter::Directory(_) => "directory",
            WorksFilter::Doi(_) => "doi",
            WorksFilter::Updates(_) => "updates",
            WorksFilter::IsUpdate => "is-update",
            WorksFilter::HasUpdatePolicy => "has-update-policy",
            WorksFilter::ContainerTitle(_) => "container-title",
            WorksFilter::CategoryName(_) => "category-name",
            WorksFilter::TypeName(_) => "type-name",
            WorksFilter::AwardNumber(_) => "award.number",
            WorksFilter::AwardFunder(_) => "award.funder",
            WorksFilter::HasAssertion => "has-assertion",
            WorksFilter::AssertionGroup(_) => "assertion-group",
            WorksFilter::Assertion(_) => "assertion",
            WorksFilter::HasAffiliation => "has-affiliation",
            WorksFilter::AlternativeId => "alternative-id",
            WorksFilter::ArticleNumber => "article-number",
            WorksFilter::HasAbstract => "has-abstract",
            WorksFilter::HasClinicalTrialNumber => "has-clinical-trial-number	",
            WorksFilter::ContentDomain(_) => "content-domain",
            WorksFilter::HasContentDomain => "has-content-domain",
            WorksFilter::HasDomainRestriction => "has-domain-restriction",
            WorksFilter::HasRelation => "has-relation",
            WorksFilter::RelationType => "relation.type",
            WorksFilter::RelationObject => "relation.object",
            WorksFilter::RelationObjectType(_) => "relation.object-type",
        }
    }
}

impl ParamFragment for WorksFilter {
    fn key(&self) -> Cow<str> {
        Cow::Borrowed(self.name())
    }

    fn value(&self) -> Option<Cow<str>> {
        match self {
            WorksFilter::Funder(s)
            | WorksFilter::Location(s)
            | WorksFilter::Prefix(s)
            | WorksFilter::Member(s)
            | WorksFilter::LicenseUrl(s)
            | WorksFilter::LicenseVersion(s)
            | WorksFilter::FullTextVersion(s)
            | WorksFilter::FullTextType(s)
            | WorksFilter::FullTextApplication(s)
            | WorksFilter::Archive(s)
            | WorksFilter::Orcid(s)
            | WorksFilter::Issn(s)
            | WorksFilter::Isbn(s)
            | WorksFilter::Directory(s)
            | WorksFilter::Doi(s)
            | WorksFilter::Updates(s)
            | WorksFilter::ContainerTitle(s)
            | WorksFilter::CategoryName(s)
            | WorksFilter::AwardNumber(s)
            | WorksFilter::TypeName(s)
            | WorksFilter::AwardFunder(s)
            | WorksFilter::AssertionGroup(s)
            | WorksFilter::Assertion(s)
            | WorksFilter::ContentDomain(s)
            | WorksFilter::RelationObjectType(s) => Some(Cow::Borrowed(s.as_str())),
            WorksFilter::ReferenceVisibility(vis) => Some(Cow::Borrowed(vis.as_str())),
            WorksFilter::FromIndexDate(d)
            | WorksFilter::UntilIndexDate(d)
            | WorksFilter::FromDepositDate(d)
            | WorksFilter::UntilDepositDate(d)
            | WorksFilter::FromUpdateDate(d)
            | WorksFilter::UntilUpdateDate(d)
            | WorksFilter::FromCreatedDate(d)
            | WorksFilter::UntilCreatedDate(d)
            | WorksFilter::FromPubDate(d)
            | WorksFilter::UntilPubDate(d)
            | WorksFilter::FromOnlinePubDate(d)
            | WorksFilter::UntilOnlinePubDate(d)
            | WorksFilter::FromPrintPubDate(d)
            | WorksFilter::UntilPrintPubDate(d)
            | WorksFilter::FromPostedDate(d)
            | WorksFilter::UntilPostedDate(d)
            | WorksFilter::FromAcceptedDate(d)
            | WorksFilter::UntilAcceptedDate(d) => {
                Some(Cow::Owned(d.format("%Y-%m-%d").to_string()))
            }
            WorksFilter::Type(t) => Some(Cow::Borrowed(t.id())),
            _ => Some(Cow::Borrowed("true")),
        }
    }
}

impl Filter for WorksFilter {}

/// Field queries are available on the `/works` route and allow for queries that match only particular fields of metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldQuery {
    /// match any only particular fields of metadata.
    pub name: String,
    /// the value of the query
    pub value: String,
}

impl FieldQuery {
    /// creates a new `Field` query for `title` and `subtitle`
    pub fn title(title: &str) -> Self {
        Self {
            name: "title".to_string(),
            value: title.to_string(),
        }
    }

    /// creates a new `Field` query for `container-title` aka `publication.name`
    pub fn container_title(container_title: &str) -> Self {
        Self {
            name: "container-title".to_string(),
            value: container_title.to_string(),
        }
    }
    /// creates a new `Field` query author given and family names
    pub fn author(author: &str) -> Self {
        Self {
            name: "author".to_string(),
            value: author.to_string(),
        }
    }
    /// creates a new `Field` query for editor given and family names
    pub fn editor(editor: &str) -> Self {
        Self {
            name: "editor".to_string(),
            value: editor.to_string(),
        }
    }
    /// creates a new `Field` query for chair given and family names
    pub fn chair(chair: &str) -> Self {
        Self {
            name: "chair".to_string(),
            value: chair.to_string(),
        }
    }
    /// creates a new `Field` query for translator given and family names
    pub fn translator(translator: &str) -> Self {
        Self {
            name: "translator".to_string(),
            value: translator.to_string(),
        }
    }
    /// creates a new `Field` query for author, editor, chair and translator given and family names
    pub fn contributor(contributor: &str) -> Self {
        Self {
            name: "contributor".to_string(),
            value: contributor.to_string(),
        }
    }
    /// creates a new `Field` query for bibliographic information, useful for citation look up.
    /// Includes titles, authors, ISSNs and publication years
    pub fn bibliographic(bibliographic: &str) -> Self {
        Self {
            name: "bibliographic".to_string(),
            value: bibliographic.to_string(),
        }
    }
    /// creates a new `Field` query for contributor affiliations
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

/// limits from where and how many `Work` items should be returned
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
                token.as_ref().map(String::as_str).unwrap_or("*")
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
///
/// Retrieve a publication by DOI
///
/// # Example
///
/// ```edition2018
/// use crossref::Works;
///
/// let works = Works::doi("10.1037/0003-066X.59.1.29");
/// ```
///
/// Target the agency of a specific publication, where the str supplied is corresponded to the publication's DOI
///
/// # Example
///
/// ```edition2018
/// use crossref::Works;
///
/// let works = Works::agency_for_doi("10.1037/0003-066X.59.1.29");
/// ```
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
    /// create a new `Works::Identifier` by converting `doi` to a `String`
    pub fn doi(doi: &str) -> Self {
        Works::Identifier(doi.to_string())
    }
    /// create a new `Works::Agency` targeting the registration agency for the DOI
    pub fn agency_for_doi(doi: &str) -> Self {
        Works::Agency(doi.to_string())
    }
}

impl CrossrefRoute for Works {
    fn route(&self) -> Result<String> {
        match self {
            Works::Identifier(s) => Ok(format!("{}/{}", Component::Works.route()?, s)),
            Works::Agency(s) => Ok(format!("{}/{}/agency", Component::Works.route()?, s)),
            Works::Query(query) => query.route(),
        }
    }
}

impl CrossrefQuery for Works {
    fn resource_component(self) -> ResourceComponent {
        ResourceComponent::Works(self)
    }
}

#[derive(Debug, Clone)]
pub enum WorkListQuery {
    Works(WorksQuery),
    Combined {
        primary_component: Component,
        ident: WorksIdentQuery,
    },
}

impl Into<WorkListQuery> for WorksQuery {
    fn into(self) -> WorkListQuery {
        WorkListQuery::Works(self)
    }
}

impl CrossrefRoute for WorkListQuery {
    fn route(&self) -> Result<String> {
        match self {
            WorkListQuery::Works(query) => query.route(),
            WorkListQuery::Combined {
                primary_component,
                ident,
            } => Ok(format!(
                "{}/{}{}",
                primary_component.route()?,
                ident.id,
                ident.query.route()?
            )),
        }
    }
}

impl CrossrefQuery for WorkListQuery {
    fn resource_component(self) -> ResourceComponent {
        match self {
            WorkListQuery::Works(query) => ResourceComponent::Works(Works::Query(query)),
            WorkListQuery::Combined {
                primary_component,
                ident,
            } => match primary_component {
                Component::Funders => ResourceComponent::Funders(Funders::Works(ident)),
                Component::Journals => ResourceComponent::Journals(Journals::Works(ident)),
                Component::Members => ResourceComponent::Members(Members::Works(ident)),
                Component::Prefixes => ResourceComponent::Prefixes(Prefixes::Works(ident)),
                Component::Types => ResourceComponent::Types(Types::Works(ident)),
                Component::Works => ResourceComponent::Works(Works::Query(ident.query)),
            },
        }
    }
}

/// Target `Works` as secondary resource component
///
/// # Example
///
/// ```edition2018
/// use crossref::{WorksIdentQuery, WorksQuery};
///
/// let combined = WorksIdentQuery::new("100000015", WorksQuery::new().query("ontologies"));
///
/// ```
/// Is equal to create a `WorksIdentQuery` from a `WorksQuery`
///
/// ```edition2018
/// use crossref::WorksQuery;
///
/// let combined = WorksQuery::new().query("ontologies").into_ident("100000015");
///
/// ```
/// helper struct to capture an id for a `Component` other than `/works` and an additional query for the `/works` route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorksIdentQuery {
    /// the id of an component item
    pub id: String,
    /// the query to filter the works results
    pub query: WorksQuery,
}

impl WorksIdentQuery {
    pub fn new<T: Into<String>>(id: T, query: WorksQuery) -> Self {
        WorksIdentQuery {
            id: id.into(),
            query,
        }
    }
}

pub trait WorksCombiner {
    fn primary_component() -> Component;

    fn ident_query(ident: WorksIdentQuery) -> Self;

    fn combined_route(ident: &WorksIdentQuery) -> Result<String> {
        Ok(format!(
            "{}/{}{}",
            Self::primary_component().route()?,
            ident.id,
            ident.query.route()?
        ))
    }

    fn work_list_query(ident: WorksIdentQuery) -> WorkListQuery {
        WorkListQuery::Combined {
            primary_component: Self::primary_component(),
            ident,
        }
    }
}

macro_rules! impl_combiner {
    ($($name:ident,)*) => {
        $(
        impl WorksCombiner for $name {
            fn primary_component() -> Component {
                Component::$name
            }

            fn ident_query(ident: WorksIdentQuery) -> Self {
                $name::Works(ident)
            }
        }
        )+
    };
}

impl_combiner!(Journals, Funders, Members, Prefixes, Types,);

impl WorksQuery {
    /// alias for creating an empty default element
    pub fn empty() -> Self {
        WorksQuery::default()
    }

    /// creates an new `WorksQuery` with the desired sample size that will result in
    /// a request for random dois
    pub fn random(len: usize) -> Self {
        WorksQuery::default().sample(len)
    }

    /// alias for creating an new default element
    pub fn new() -> Self {
        WorksQuery::default()
    }

    /// add a new free form query
    pub fn sample(mut self, len: usize) -> Self {
        self.sample = Some(len);
        self
    }

    /// add a new free form query
    pub fn query(mut self, query: &str) -> Self {
        self.free_form_queries.push(query.to_string());
        self
    }

    /// Create a new query for the topics renear+ontologies
    ///
    /// # Example
    ///
    /// ```edition2018
    /// use crossref::WorksQuery;
    ///
    /// let query = WorksQuery::new().queries(&["renear", "ontologies"]);
    /// ```
    /// add a bunch of free form query terms
    pub fn queries<T: ToString>(mut self, queries: &[T]) -> Self {
        self.free_form_queries
            .extend(queries.iter().map(T::to_string));
        self
    }

    /// add a new field query form query
    pub fn field_query(mut self, query: FieldQuery) -> Self {
        self.field_queries.push(query);
        self
    }

    /// ```edition2018
    /// use crossref::{FieldQuery,WorksQuery};
    ///
    /// let query = WorksQuery::new().field_queries(vec![FieldQuery::title("room at the bottom"), FieldQuery::author("richard feynman")]);
    /// ```
    /// add a bunch of free form query terms
    pub fn field_queries(mut self, queries: Vec<FieldQuery>) -> Self {
        self.field_queries.extend(queries.into_iter());
        self
    }

    /// add a new filter to the query
    pub fn filter(mut self, filter: WorksFilter) -> Self {
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

    /// set the cursor for result control deep paging
    pub fn next_cursor(mut self, cursor: &str) -> Self {
        let rows = match self.result_control {
            Some(WorkResultControl::Standard(ResultControl::Rows(rows))) => Some(rows),
            _ => None,
        };
        self.result_control = Some(WorkResultControl::Cursor {
            token: Some(cursor.to_string()),
            rows,
        });
        self
    }

    /// set an empty cursor
    pub fn new_cursor(mut self) -> Self {
        self.result_control = Some(WorkResultControl::new_cursor());
        self
    }
    /// set result control option to query
    pub fn result_control(mut self, result_control: WorkResultControl) -> Self {
        self.result_control = Some(result_control);
        self
    }

    /// Wrap the query in a combined query
    ///
    /// # Example
    /// Create a Funders Query that targets all works of a funder if
    ///
    /// ```edition2018
    /// # use crossref::{WorksQuery, Funders};
    /// let funders_query: Funders = WorksQuery::new().into_combined("funder_id");
    /// ```
    pub fn into_combined<W: WorksCombiner>(self, id: &str) -> W {
        W::ident_query(self.into_ident(id))
    }

    pub fn into_ident(self, id: &str) -> WorksIdentQuery {
        WorksIdentQuery::new(id, self)
    }
}

/// Used to construct a query that targets crossref `Works` elements
///
/// # Example
///
/// ```edition2018
/// use crossref::{Order, WorksQuery};
///
/// // create a new query for topcis machine+learning ordered desc
/// let query = WorksQuery::new().query("machine learning").order(Order::Desc);
/// ```
///
/// Each query parameter is ANDed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorksQuery {
    /// search by non specific query
    pub free_form_queries: Vec<String>,
    /// match only particular fields of metadata
    pub field_queries: Vec<FieldQuery>,
    /// filter to apply while querying
    pub filter: Vec<WorksFilter>,
    /// sort results by a certain field and
    pub sort: Option<Sort>,
    /// set the sort order to `asc` or `desc`
    pub order: Option<Order>,
    /// enable facet information in responses
    pub facets: Vec<FacetCount>,
    /// deep page through `/works` result sets
    pub result_control: Option<WorkResultControl>,
    /// request random dois
    /// if set all other parameters are ignored
    pub sample: Option<usize>,
}

impl CrossrefRoute for WorksQuery {
    fn route(&self) -> Result<String> {
        let mut params = Vec::new();

        if let Some(sample) = self.sample {
            return Ok(format!("sample={}", sample));
        }

        if !self.free_form_queries.is_empty() {
            params.push(Cow::Owned(format!(
                "query={}",
                format_queries(&self.free_form_queries)
            )));
        }
        if !self.field_queries.is_empty() {
            params.extend(self.field_queries.iter().map(CrossrefQueryParam::param))
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

        Ok(format!(
            "{}?{}",
            Component::Works.route()?,
            params.join("&")
        ))
    }
}

impl CrossrefParams for WorksQuery {
    type Filter = WorksFilter;

    fn query_terms(&self) -> &[String] {
        &self.free_form_queries
    }
    fn filters(&self) -> &[Self::Filter] {
        &self.filter
    }
    fn sort(&self) -> Option<&Sort> {
        self.sort.as_ref()
    }
    fn order(&self) -> Option<&Order> {
        self.order.as_ref()
    }
    fn facets(&self) -> &[FacetCount] {
        &self.facets
    }
    fn result_control(&self) -> Option<&ResultControl> {
        if let Some(WorkResultControl::Standard(ref std)) = self.result_control {
            Some(std)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_works_ident() {
        let works = Works::doi("10.1037/0003-066X.59.1.29");

        assert_eq!("/works/10.1037/0003-066X.59.1.29", &works.route().unwrap())
    }
}
