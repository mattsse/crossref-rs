use crate::model::*;
use crate::types::Types;
use chrono::NaiveDate;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Facet {}

/// Filters allow you to narrow queries. All filter results are lists
#[derive(Debug, PartialEq, Eq, Clone)]
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
    Type(Option<Types>),
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
    ContainerTitle,
    /// metadata for records with an exact matching category label.
    /// Category labels come from [this list](https://www.elsevier.com/solutions/scopus/content) published by Scopus
    CategoryName(String),
    /// metadata for records with an exacty matching type label
    TypeName,
    /// metadata for records with a matching award number.
    /// Optionally combine with `award.funder`
    AwardNumber(String),
    /// metadata for records with an award with matching funder.
    /// Optionally combine with `award.number`
    AwardFunder(String),
    /// metadata for records with any assertions
    HasAssertion,
    /// metadata for records with an assertion in a particular group
    AssertionGroup,
    /// metadata for records with a particular named assertion
    Assertion,
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
    ContentDomain,
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
    RelationObjectType,
}

/// filters supported for the `/members` route
#[derive(Debug, PartialEq, Eq, Clone)]
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

/// filters supported for the /funders route
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FundersFilter {
    /// funders located in specified country
    Location(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Visibility {
    Open,
    Limited,
    Closed,
}

/// Results from a listy response can be sorted by applying the sort and order parameters.
/// Order sets the result ordering, either asc or desc. Sort sets the field by which results will be sorted.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SortOption {
    /// Sort by relevance score
    Score,
    /// Sort by date of most recent change to metadata. Currently the same as [Deposited]
    Updated,
    /// Sort by time of most recent deposit
    Deposited,
    /// Sort by time of most recent index
    Indexed,
    /// Sort by publication date
    Published,
    /// Sort by print publication date
    PublishedPrint,
    /// Sort by online publication date
    PublishedOnline,
    /// Sort by issued date (earliest known publication date)
    Issued,
    /// Sort by number of times this DOI is referenced by other Crossref DOIs
    IsReferencedByCount,
    /// Sort by number of references included in the references section of the document identified by this DOI
    ReferenceCount,
}

#[derive(Debug, Clone)]
pub enum Query {
    /// free form search query
    FreeForm(String),
    /// match any only particular fields of metadata
    Field { name: String, value: String },
}

impl Query {
    /// creates a new [Field] query for `title` and `subtitle`
    fn title(title: &str) -> Self {
        Query::Field {
            name: "title".to_string(),
            value: title.to_string(),
        }
    }

    /// creates a new [Field] query for `container-title` aka `publication.name`
    fn container_title(container_title: &str) -> Self {
        Query::Field {
            name: "container-title".to_string(),
            value: container_title.to_string(),
        }
    }
    /// creates a new [Field] query author given and family names
    fn author(author: &str) -> Self {
        Query::Field {
            name: "author".to_string(),
            value: author.to_string(),
        }
    }
    /// creates a new [Field] query for editor given and family names
    fn editor(editor: &str) -> Self {
        Query::Field {
            name: "editor".to_string(),
            value: editor.to_string(),
        }
    }
    /// creates a new [Field] query for chair given and family names
    fn chair(chair: &str) -> Self {
        Query::Field {
            name: "chair".to_string(),
            value: chair.to_string(),
        }
    }
    /// creates a new [Field] query for translator given and family names
    fn translator(translator: &str) -> Self {
        Query::Field {
            name: "translator".to_string(),
            value: translator.to_string(),
        }
    }
    /// creates a new [Field] query for author, editor, chair and translator given and family names
    fn contributor(contributor: &str) -> Self {
        Query::Field {
            name: "contributor".to_string(),
            value: contributor.to_string(),
        }
    }
    /// creates a new [Field] query for bibliographic information, useful for citation look up.
    /// Includes titles, authors, ISSNs and publication years
    fn bibliographic(bibliographic: &str) -> Self {
        Query::Field {
            name: "bibliographic".to_string(),
            value: bibliographic.to_string(),
        }
    }
    /// creates a new [Field] query for contributor affiliations
    fn affiliation(affiliation: &str) -> Self {
        Query::Field {
            name: "affiliation".to_string(),
            value: affiliation.to_string(),
        }
    }
}

pub trait CrossQuery {}

#[derive(Debug, Clone)]
pub struct Params {
    pub queries: Vec<Query>,
}

impl Params {
    fn encode(&self, base_url: &str) -> String {
        unimplemented!()
    }
}

pub struct QueryBuilder {}
