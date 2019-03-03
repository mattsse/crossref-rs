// see https://github.com/Crossref/rest-api-doc/blob/master/api_format.md

use chrono::NaiveDate;

/// A hashmap containing relation name, Relation pairs.
pub type Relations = std::collections::HashMap<String, Relation>;

/// Helper struct to represent dates in the cross ref api as nested arrays of numbers
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DateParts(pub Vec<Vec<u32>>);

impl DateParts {
    /// converts the nested array of numbers into the corresponding [DateField]
    /// standalone years are allowed.
    /// if an array is empty, [None] will be returned
    pub fn as_date(&self) -> Option<DateField> {
        /// converts an array of numbers into chrono [NaiveDate] if it contains at least a single value
        fn naive(v: &[u32]) -> Option<NaiveDate> {
            match v.len() {
                0 => None,
                1 => Some(NaiveDate::from_ymd(v[0] as i32, 0, 0)),
                2 => Some(NaiveDate::from_ymd(v[0] as i32, v[1], 0)),
                3 => Some(NaiveDate::from_ymd(v[0] as i32, v[1], v[2])),
                _ => None,
            }
        }

        match self.0.len() {
            0 => None,
            1 => Some(DateField::Single(naive(&self.0[0])?)),
            2 => Some(DateField::Range {
                from: naive(&self.0[0])?,
                to: naive(&self.0[1])?,
            }),
            _ => Some(DateField::Multi(
                self.0
                    .iter()
                    .map(|x| naive(x))
                    .collect::<Option<Vec<_>>>()?,
            )),
        }
    }
}

/// the main return type of the crossref api
/// represents a publication
/// based on the [crossref rest-api-doc](https://github.com/CrossRef/rest-api-doc/blob/master/api_format.md#work)
/// with minor adjustments
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Work {
    /// Name of work's publisher
    pub publisher: String,
    /// Work titles, including translated titles
    pub title: Vec<String>,
    /// Work titles in the work's original publication language
    pub original_title: Option<Vec<String>>,
    /// the language of this work
    pub language: Option<String>,
    /// Abstract as a JSON string or a JATS XML snippet encoded into a JSON string
    pub short_title: Option<Vec<String>>,
    /// Abstract as a JSON string or a JATS XML snippet encoded into a JSON string
    #[serde(rename = "abstract")]
    pub abstract_: Option<String>,
    /// Count of outbound references deposited with Crossref
    pub references_count: i32,
    /// Count of inbound references deposited with Crossref
    pub is_referenced_by_count: i32,
    /// Currently always `Crossref`
    pub source: String,

    pub journal_issue: Option<Issue>,
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
    /// the day this work entry was created
    pub created: Option<Date>,
    /// Date on which the DOI was first registered
    pub date: Option<Date>,
    /// Date on which the work metadata was most recently updated
    pub deposited: Option<Date>,
    /// the works crossref score
    pub score: Option<i32>,
    /// Date on which the work metadata was most recently indexed.
    /// Re-indexing does not imply a metadata change, see `deposited` for the most recent metadata change date
    pub indexed: Date,
    /// Earliest of `published-print` and `published-online`
    pub issued: PartialDate,
    /// ate on which posted content was made available online
    pub posted: Option<PartialDate>,
    /// Date on which a work was accepted, after being submitted, during a submission process
    pub accepted: Option<PartialDate>,
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
    /// the number of the corresponding article
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
    pub alternative_id: Option<Vec<String>>,
    /// List of references made by the work
    pub reference: Option<Vec<Reference>>,
    /// Information on domains that support Crossmark for this work
    pub content_domain: Option<ContentDomain>,
    /// Relations to other works
    pub relation: Option<Relations>,
    /// Peer review metadata
    pub review: Option<Relations>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Affiliation {
    /// the affiliation's name
    pub name: String,
}

/// represents full date information for an item
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Date {
    /// Contains an ordered array of year, month, day of month.
    /// Only year is required. Note that the field contains a nested array,
    /// e.g. [ [ 2006, 5, 19 ] ] to conform to citeproc JSON dates
    pub date_parts: DateParts,
    /// Seconds since UNIX epoch
    pub timestamp: usize,
    /// ISO 8601 date time
    pub date_time: String,
}

impl Date {
    /// converts the nested array of numbers into the correct representation of chrono [NaiveDate]
    pub fn as_date_field(&self) -> Option<DateField> {
        self.date_parts.as_date()
    }
}

/// represents an incomplete date only consisting of year or year and month
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PartialDate {
    /// Contains an ordered array of year, month, day of month.
    /// Only year is required
    /// e.g. [ [ 2006 ] ] to conform to citeproc JSON dates
    #[serde(rename = "date-parts")]
    pub date_parts: DateParts,
}

impl PartialDate {
    /// converts the nested array of numbers into the correct representation of chrono [NaiveDate]
    pub fn as_date_field(&self) -> Option<DateField> {
        self.date_parts.as_date()
    }
}

/// Helper struct to capture all possible occurrences of dates in the crossref api, a nested Vec of numbers
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum DateField {
    /// only a single date vector
    Single(NaiveDate),
    /// two date vectors represent a range
    Range { from: NaiveDate, to: NaiveDate },
    /// more than two date vectors are present
    Multi(Vec<NaiveDate>),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Issue {
    /// Date on which the work was published in print
    pub published_print: Option<PartialDate>,
    /// Date on which the work was published online
    pub published_online: Option<PartialDate>,
    /// Issue number of an article's journal
    pub issue: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AssertionGroup {
    pub name: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ISSN {
    pub value: String,
    /// One of `eissn`, `pissn` or `lissn`
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContentDomain {
    pub domain: Vec<String>,
    pub crossmark_restriction: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Relation {
    pub id_type: Option<String>,
    pub id: Option<String>,
    pub asserted_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;
    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct Demo {
        pub date_parts: DateParts,
    }
    #[test]
    fn date_parts_serde() {
        let demo = Demo {
            date_parts: DateParts(vec![vec![2017, 10, 11]]),
        };
        let expected = r##"{"date_parts":[[2017,10,11]]}"##;
        assert_eq!(expected, &to_string(&demo).unwrap());
        assert_eq!(demo, from_str::<Demo>(expected).unwrap());
    }

    #[test]
    fn serialize_work() {
        let work_str = r##"{
    "indexed": {
      "date-parts": [
        [
          2019,
          2,
          26
        ]
      ],
      "date-time": "2019-02-26T10:43:14Z",
      "timestamp": 1551177794515
    },
    "reference-count": 105,
    "publisher": "American Psychological Association (APA)",
    "issue": "1",
    "content-domain": {
      "domain": [],
      "crossmark-restriction": false
    },
    "short-container-title": [
      "American Psychologist"
    ],
    "DOI": "10.1037/0003-066x.59.1.29",
    "type": "journal-article",
    "created": {
      "date-parts": [
        [
          2004,
          1,
          21
        ]
      ],
      "date-time": "2004-01-21T14:31:19Z",
      "timestamp": 1074695479000
    },
    "page": "29-40",
    "source": "Crossref",
    "is-referenced-by-count": 84,
    "title": [
      "How the Mind Hurts and Heals the Body."
    ],
    "prefix": "10.1037",
    "volume": "59",
    "author": [
      {
        "given": "Oakley",
        "family": "Ray",
        "sequence": "first",
        "affiliation": []
      }
    ],
    "member": "15",
    "published-online": {
      "date-parts": [
        [
          2004
        ]
      ]
    },
    "container-title": [
      "American Psychologist"
    ],
    "original-title": [],
    "language": "en",
    "link": [
      {
        "URL": "http://psycnet.apa.org/journals/amp/59/1/29.pdf",
        "content-type": "unspecified",
        "content-version": "vor",
        "intended-application": "similarity-checking"
      }
    ],
    "deposited": {
      "date-parts": [
        [
          2018,
          4,
          8
        ]
      ],
      "date-time": "2018-04-08T18:56:17Z",
      "timestamp": 1523213777000
    },
    "score": 1,
    "subtitle": [],
    "short-title": [],
    "issued": {
      "date-parts": [
        [
          2004
        ]
      ]
    },
    "references-count": 105,
    "journal-issue": {
      "published-online": {
        "date-parts": [
          [
            2004
          ]
        ]
      },
      "issue": "1"
    },
    "alternative-id": [
      "2004-10043-004",
      "14736318"
    ],
    "URL": "http://dx.doi.org/10.1037/0003-066x.59.1.29",
    "relation": {},
    "ISSN": [
      "1935-990X",
      "0003-066X"
    ],
    "issn-type": [
      {
        "value": "0003-066X",
        "type": "print"
      },
      {
        "value": "1935-990X",
        "type": "electronic"
      }
    ]
  }
"##;

        let work: Work = from_str(work_str).unwrap();
    }

}
