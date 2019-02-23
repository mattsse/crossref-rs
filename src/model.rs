// see https://github.com/Crossref/rest-api-doc/blob/master/api_format.md
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Work {
    /// Name of work's publisher
    pub publisher: String,
    /// Work titles, including translated titles
    pub title: Vec<String>,
    /// Work titles in the work's original publication language
    pub original_title: Option<Vec<String>>,
    /// Abstract as a JSON string or a JATS XML snippet encoded into a JSON string
    pub short_title: Option<Vec<String>>,
    /// Abstract as a JSON string or a JATS XML snippet encoded into a JSON string
    #[serde(rename = "abstract")]
    pub abstract_: String,
    /// Count of outbound references deposited with Crossref
    #[serde(rename = "references-count")]
    pub references_count: i32,
    /// Count of inbound references deposited with Crossref
    pub is_referenced_by_count: i32,
    /// Currently always `Crossref`
    pub source: String,
    /// DOI prefix identifier of the form `http://id.crossref.org/prefix/DOI_PREFIX`
    pub prefix: String,
    /// DOI of the work
    #[serde(rename = "DOI")]
    pub doi: String,
    /// URL form of the work's DOI
    #[serde(rename = "URL")]
    pub url: String,
    /// Member identifier of the form `http://id.crossref.org/member/MEMBER_ID`
    pub member: String,
    /// Enumeration, one of the type ids from `https://api.crossref.org/v1/types`
    #[serde(rename = "type")]
    pub type_: String,
    /// Date on which the DOI was first registered
    pub date: Date,
    /// Date on which the work metadata was most recently updated
    pub deposited: Date,
    /// Date on which the work metadata was most recently indexed.
    /// Re-indexing does not imply a metadata change, see `deposited` for the most recent metadata change date
    pub indexed: Date,
    /// Earliest of `published-print` and `published-online`
    pub issued: PartialDate,
    /// ate on which posted content was made available online
    pub posted: PartialDate,
    /// Date on which a work was accepted, after being submitted, during a submission process
    pub accepted: PartialDate,
    /// Work subtitles, including original language and translated
    pub subtitle: Option<Vec<String>>,
    /// Full titles of the containing work (usually a book or journal)
    pub container_title: Option<Vec<String>>,
    /// Abbreviated titles of the containing work
    pub short_container_title: Option<Vec<String>>,
    /// Group title for posted content
    pub group_title: Option<String>,
    /// Issue number of an article's journal
    pub issue: Option<String>,
    /// Volume number of an article's journal
    pub volume: Option<String>,
    /// Pages numbers of an article within its journal
    pub page: Option<String>,
    pub article_number: Option<String>,
    /// Date on which the work was published in print
    pub published_print: Option<PartialDate>,
    /// Date on which the work was published online
    pub published_online: Option<PartialDate>,
    /// Subject category names, a controlled vocabulary from Sci-Val.
    /// Available for most journal articles
    pub subject: Option<Vec<String>>,
    #[serde(rename = "ISSN")]
    pub issn: Option<Vec<String>>,
    /// List of ISSNs with ISSN type information
    pub issn_type: Option<Vec<ISSN>>,
    #[serde(rename = "ISBN")]
    pub isbn: Option<Vec<String>>,
    pub archive: Option<Vec<String>>,
    pub license: Option<Vec<License>>,
    pub funder: Option<Vec<Funder>>,
    pub assertion: Option<Vec<Assertion>>,
    pub author: Option<Vec<Contributor>>,
    pub editor: Option<Vec<Contributor>>,
    pub chair: Option<Vec<Contributor>>,
    pub translator: Option<Vec<Contributor>>,
    pub update_to: Option<Vec<Update>>,
    /// Link to an update policy covering Crossmark updates for this work
    pub update_policy: Option<String>,
    /// URLs to full-text locations
    pub link: Option<Vec<ResourceLink>>,
    pub clinical_trial_number: Option<Vec<ClinicalTrialNumber>>,
    /// Other identifiers for the work provided by the depositing member
    pub alternative_id: Option<String>,
    /// List of references made by the work
    pub reference: Option<Vec<Reference>>,
    /// Information on domains that support Crossmark for this work
    pub content_domain: Option<ContentDomain>,
    /// Relations to other works
    pub relation: Option<Relations>,
    /// Peer review metadata
    pub review: Option<Relations>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Funder {
    /// Funding body primary name
    pub name: String,
    /// Optional [Open Funder Registry](http://www.crossref.org/fundingdata/registry.html) DOI uniquely identifing the funding body
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    /// Award number(s) for awards given by the funding body
    pub award: Option<Vec<String>>,
    /// Either `crossref` or `publisher`
    #[serde(rename = "doi-asserted-by")]
    pub doi_asserted_by: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ClinicalTrialNumber {
    /// Identifier of the clinical trial
    #[serde(rename = "clinical-trial-number")]
    pub clinical_trial_number: String,
    /// DOI of the clinical trial regsitry that assigned the trial number
    pub registry: String,
    /// One of `preResults`, `results` or `postResults`
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Contributor {
    pub family: String,
    pub given: Option<String>,
    /// URL-form of an [ORCID](http://orcid.org) identifier
    #[serde(rename = "ORCID")]
    pub orcid: Option<String>,
    /// If true, record owner asserts that the ORCID user completed ORCID OAuth authentication
    #[serde(rename = "authenticated-orcid")]
    pub authenticated_orcid: Option<bool>,
    pub affiliation: Option<Vec<Affiliation>>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Affiliation {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Date {
    /// Contains an ordered array of year, month, day of month.
    /// Only year is required. Note that the field contains a nested array,
    /// e.g. [ [ 2006, 5, 19 ] ] to conform to citeproc JSON dates
    pub date_parts: Vec<i32>,
    /// Seconds since UNIX epoch
    pub timestamp: i32,
    /// ISO 8601 date time
    pub date_time: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PartialDate {
    /// Contains an ordered array of year, month, day of month.
    /// Only year is required. Note that the field contains a nested array,
    /// e.g. [ [ 2006, 5, 19 ] ] to conform to citeproc JSON dates
    #[serde(rename = "date-parts")]
    pub date_parts: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Update {
    /// Date on which the update was published
    pub updated: PartialDate,
    /// DOI of the updated work
    #[serde(rename = "DOI")]
    pub doi: String,
    /// The type of update, for example retraction or correction
    #[serde(rename = "type")]
    pub type_: String,
    /// A display-friendly label for the update type
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Assertion {
    pub name: String,
    pub value: String,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    pub explanation: Option<String>,
    pub label: Option<String>,
    pub order: Option<i32>,
    pub group: Option<AssertionGroup>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AssertionGroup {
    pub name: String,
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct License {
    /// Either `vor` (version of record,) `am` (accepted manuscript) or `unspecified`
    pub content_version: String,
    /// Number of days between the publication date of the work and the start date of this license
    pub delay_in_days: i32,
    /// Date on which this license begins to take effect
    pub start: PartialDate,
    /// Link to a web page describing this license
    #[serde(rename = "URL")]
    pub url: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResourceLink {
    /// Either `text-mining`, `similarity-checking` or `unspecified`
    pub intended_application: String,
    /// Either `vor` (version of record,) `am` (accepted manuscript) or `unspecified`
    pub content_version: String,
    /// Direct link to a full-text download location
    #[serde(rename = "URL")]
    pub url: String,
    /// Content type (or MIME type) of the full-text object
    pub content_type: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Reference {
    pub key: String,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    /// One of `crossref` or `publisher`
    pub doi_asserted_by: Option<String>,
    pub issue: Option<String>,
    pub first_page: Option<String>,
    pub volume: Option<String>,
    pub edition: Option<String>,
    pub component: Option<String>,
    pub standard_designator: Option<String>,
    pub standards_body: Option<String>,
    pub author: Option<String>,
    pub year: Option<String>,
    pub unstructured: Option<String>,
    pub journal_title: Option<String>,
    pub article_title: Option<String>,
    pub series_title: Option<String>,
    pub volume_title: Option<String>,
    #[serde(rename = "ISSN")]
    pub issn: Option<String>,
    /// One of `pissn` or `eissn`
    pub issn_type: Option<String>,
    #[serde(rename = "ISBN")]
    pub isbn: Option<String>,
    pub isbn_type: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ISSN {
    pub value: String,
    /// One of `eissn`, `pissn` or `lissn`
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContentDomain {
    pub domain: Vec<String>,
    pub crossmark_restriction: bool,
}

/// A hashmap containing relation name, Relation pairs.
type Relations = HashMap<String, Relation>;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Relation {
    pub id_type: Option<String>,
    pub id: Option<String>,
    pub asserted_by: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Review {
    pub running_number: Option<String>,
    pub revision_round: Option<String>,
    /// One of `pre-publication` or `post-publication`
    pub stage: Option<String>,
    /// One of `major-revision` or `minor-revision` or `reject` or `reject-with-resubmit` or `accept`
    pub recommendation: Option<String>,
    /// One of `referee-report` or `editor-report` or `author-comment` or `community-comment` or `aggregate`
    #[serde(rename = "type")]
    pub type_: String,
    pub competing_interest_statement: Option<String>,
    pub language: Option<String>,
}
