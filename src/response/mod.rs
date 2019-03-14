use crate::query::facet::Facet;
use crate::query::facet::FacetCount;
use crate::query::Visibility;
use crate::response::work::*;
use serde::de::{self, Deserialize, Deserializer};

use serde_json::{from_value, Value};
use std::collections::HashMap;
use std::fmt;

/// provides the types for a work response
pub mod work;

pub use crate::response::work::{Work, WorkList};

/// Represents the whole crossref response for a any request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Response {
    /// the status of the request
    pub status: String,
    /// the type of the response message holds
    pub message_type: MessageType,
    /// the version of the service created this message
    #[serde(default = "default_msg_version")]
    pub message_version: String,
    /// the actual message of the response
    pub message: Option<Message>,
}

/// at some routes the `msg_version` is missing, this returns the default version for a crossref response
fn default_msg_version() -> String {
    "1.0.0".to_string()
}

/// this macro helps to generate a function that checks whether the message is of a specific type
macro_rules! impl_msg_helper {
    (single: $($name:ident -> $ident:ident,)*) => {
    $(
        /// checks if the message holds the variant
        pub fn $name(&self) -> bool {
           if let Some(Message::$ident(_)) = &self.message {
               true
           } else {
               false
           }
        }
    )+
    };
}

impl Response {
    impl_msg_helper!(single:
        is_work_ageny -> WorkAgency,
        is_funder -> Funder,
        is_prefix -> Prefix,
        is_work -> Work,
        is_type -> Type,
        is_journal -> Journal,
        is_member -> Member,
        is_validation_failure -> ValidationFailure,
        is_type_list -> TypeList,
        is_work_list -> WorkList,
        is_member_list -> MemberList,
        is_journal_list -> JournalList,
        is_funder_list -> FunderList,
    );

    /// checks whether the `message` holds a variant of `RouteNotFound`
    pub fn is_route_not_found(&self) -> bool {
        match &self.message {
            Some(Message::RouteNotFound) => true,
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct ResponseFragment {
            status: String,
            message_type: MessageType,
            #[serde(default = "default_msg_version")]
            message_version: String,
            message: Option<Value>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct ListResp {
            #[serde(default)]
            facets: FacetMap,
            next_cursor: Option<String>,
            total_results: usize,
            items_per_page: Option<usize>,
            query: Option<QueryResponse>,
            items: Value,
        }

        let fragment = ResponseFragment::deserialize(deserializer).unwrap();

        macro_rules! msg_arm {
            ($ident:ident, $value:expr) => {{
                Message::$ident(::serde_json::from_value($value).map_err(de::Error::custom)?)
            }};
            ($ident:ident, $value:expr, $ty:ty) => {{
                let list_resp: ListResp =
                    ::serde_json::from_value($value).map_err(de::Error::custom)?;
                let items: Vec<$ty> =
                    ::serde_json::from_value(list_resp.items).map_err(de::Error::custom)?;
                Message::$ident($ident {
                    facets: list_resp.facets,
                    total_results: list_resp.total_results,
                    items_per_page: list_resp.items_per_page,
                    query: list_resp.query,
                    items,
                })
            }};
        }

        fn work_list(msg: Value) -> Result<Message, serde_json::Error> {
            let list_resp: ListResp = ::serde_json::from_value(msg)?;
            let items: Vec<Work> = ::serde_json::from_value(list_resp.items)?;

            Ok(Message::WorkList(WorkList {
                facets: list_resp.facets,
                total_results: list_resp.total_results,
                items_per_page: list_resp.items_per_page,
                query: list_resp.query,
                items,
                next_cursor: list_resp.next_cursor,
            }))
        }

        let message = match fragment.message {
            Some(msg) => Some(match &fragment.message_type {
                MessageType::ValidationFailure => msg_arm!(ValidationFailure, msg),
                MessageType::WorkAgency => msg_arm!(WorkAgency, msg),
                MessageType::Prefix => msg_arm!(Prefix, msg),
                MessageType::Type => msg_arm!(Type, msg),
                MessageType::TypeList => msg_arm!(TypeList, msg, CrossrefType),
                MessageType::Work => msg_arm!(Work, msg),
                MessageType::WorkList => work_list(msg).map_err(de::Error::custom)?,
                MessageType::Member => msg_arm!(Member, msg),
                MessageType::MemberList => msg_arm!(MemberList, msg, Member),
                MessageType::Journal => msg_arm!(Journal, msg),
                MessageType::JournalList => msg_arm!(JournalList, msg, Journal),
                MessageType::Funder => msg_arm!(Funder, msg),
                MessageType::FunderList => msg_arm!(FunderList, msg, Funder),
                MessageType::RouteNotFound => Message::RouteNotFound,
            }),
            _ => None,
        };
        Ok(Response {
            status: fragment.status,
            message_type: fragment.message_type,
            message_version: fragment.message_version,
            message,
        })
    }
}

macro_rules! impl_list_response {
    ($($name:ident<$ty:ty>,)*) => {
    $(
        #[derive(Debug, Clone, Deserialize, Serialize)]
        #[serde(rename_all = "kebab-case")]
        #[allow(missing_docs)]
        pub struct $name {
             /// if facets where part in the request they are also included in the response
            #[serde(default)]
            pub facets: FacetMap,
            /// the number of items that match the response
            pub total_results: usize,
            /// crossref responses for large number of items are divided in pages, number of elements to expect in `items`
            pub items_per_page: Option<usize>,
            /// if a query was set in the request, this will also be part in the response
            pub query: Option<QueryResponse>,
            /// all actual message items of the response
            pub items: Vec<$ty>,
        }
    )+
    };
}
impl_list_response!(
    TypeList<CrossrefType>,
    MemberList<Member>,
    JournalList<Journal>,
    FunderList<Funder>,
);

/// the different payloads of a response
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Message {
    /// if a request failed on the server side
    ValidationFailure(Vec<Failure>),
    /// a route could not be found on the server side
    RouteNotFound,
    /// the agency for a specific work
    WorkAgency(WorkAgency),
    /// metadata data for the DOI owner prefix
    Prefix(Prefix),
    /// a valid work type
    Type(CrossrefType),
    /// a list of valid work types
    TypeList(TypeList),
    /// a publication(journal, articles...)
    Work(Box<Work>),
    /// a list of publications
    WorkList(WorkList),
    /// a crossref member (mostly publishers)
    Member(Box<Member>),
    /// a list of crossref members
    MemberList(MemberList),
    /// a Journal publication
    Journal(Box<Journal>),
    /// list of journal publications
    JournalList(JournalList),
    /// a funder in the [funder registry](https://github.com/Crossref/open-funder-registry)
    Funder(Box<Funder>),
    /// a list of funder
    FunderList(FunderList),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(missing_docs)]
pub struct CrossrefType {
    pub id: String,
    /// Name of work's publisher
    pub label: String,
}

impl Into<CrossrefType> for crate::query::types::Type {
    fn into(self) -> CrossrefType {
        CrossrefType {
            id: self.id().to_string(),
            label: self.label().to_string(),
        }
    }
}

/// response item for the `/works/{id}/agency` route
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkAgency {
    /// the DOI fo the work that belongs to the `agency`
    #[serde(rename = "DOI")]
    doi: String,
    /// the agency that owns the work with `doi`
    agency: Agency,
}

/// response item for the `/prefix/{id}/` route
#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(missing_docs)]
pub struct Prefix {
    pub member: String,
    pub name: String,
    pub prefix: String,
}

/// all possible `message-type` of a response
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[allow(missing_docs)]
pub enum MessageType {
    WorkAgency,
    Funder,
    Prefix,
    Member,
    Work,
    WorkList,
    FunderList,
    Type,
    TypeList,
    MemberList,
    Journal,
    JournalList,
    ValidationFailure,
    RouteNotFound,
}

impl MessageType {
    /// the type identifier for a message
    pub fn as_str(&self) -> &str {
        match self {
            MessageType::WorkAgency => "work-agency",
            MessageType::Funder => "funder",
            MessageType::Prefix => "prefix",
            MessageType::Member => "member",
            MessageType::MemberList => "member-list",
            MessageType::Work => "work",
            MessageType::WorkList => "work-list",
            MessageType::FunderList => "funder-list",
            MessageType::Type => "type",
            MessageType::TypeList => "type-list",
            MessageType::Journal => "journal",
            MessageType::JournalList => "journal-list",
            MessageType::ValidationFailure => "validation-failure",
            MessageType::RouteNotFound => "route-not-found",
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

/// if a query was set in the request then it is also part of the result
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct QueryResponse {
    /// from which the returned items start
    pub start_index: usize,
    /// the terms that were initially set in the request query
    pub search_terms: Option<String>,
}

/// facets are returned as map
pub type FacetMap = HashMap<String, FacetItem>;

/// if a `facet` was set in a request `FacetMap` will be  in a `List` response as additional field of the message
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FacetItem {
    /// represents the length of `values`
    pub value_count: usize,
    /// contains the
    pub values: HashMap<String, usize>,
}

/// response item if a request could be processed
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Failure {
    /// identifier for a failue like `parameter-not-found`
    #[serde(rename = "type")]
    type_: String,
    /// value that caused the failure
    value: String,
    /// the message from the server
    message: String,
}

/// response item for the `/funder/{id}` route
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
#[allow(missing_docs)]
pub struct Funder {
    pub hierarchy_names: HashMap<String, Option<String>>,
    pub hierarchy: HashMap<String, HashMap<String, HashMap<String, bool>>>,
    pub id: String,
    pub location: String,
    pub work_count: Option<usize>,
    pub descendant_work_count: Option<usize>,
    pub descendants: Vec<String>,
    pub name: String,
    pub alt_names: Vec<String>,
    pub uri: String,
    pub replaces: Vec<String>,
    pub replaced_by: Vec<String>,
    pub tokens: Vec<String>,
}

/// response item for the `/member/{id}` route
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
#[allow(missing_docs)]
pub struct Member {
    pub primary_name: String,
    pub last_status_check_time: usize,
    pub counts: Counts,
    pub breakdowns: Breakdowns,
    pub prefixes: Vec<String>,
    pub coverage: Coverage,
    pub prefix: Vec<RefPrefix>,
    pub id: usize,
    pub tokens: Vec<String>,
    pub counts_type: HashMap<String, HashMap<String, usize>>,
    pub coverage_type: Value,
    pub flags: HashMap<String, bool>,
    pub location: String,
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
#[allow(missing_docs)]
pub struct Counts {
    pub total_dois: usize,
    pub current_dois: usize,
    pub backfile_dois: usize,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
#[allow(missing_docs)]
pub struct Breakdowns {
    pub dois_by_issued_year: Vec<Vec<u32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
#[allow(missing_docs)]
pub struct Coverage {
    pub affiliations_current: f32,
    pub similarity_checking_current: f32,
    pub funders_backfile: f32,
    pub licenses_backfile: f32,
    pub funders_current: f32,
    pub affiliations_backfile: f32,
    pub resource_links_backfile: f32,
    pub orcids_backfile: f32,
    pub update_policies_current: f32,
    pub open_references_backfile: f32,
    pub orcids_current: f32,
    pub similarity_checking_backfile: f32,
    pub references_backfile: f32,
    pub award_numbers_backfile: f32,
    pub update_policies_backfile: f32,
    pub licenses_current: f32,
    pub award_numbers_current: f32,
    pub abstracts_backfile: f32,
    pub resource_links_current: f32,
    pub abstracts_current: f32,
    pub open_references_current: f32,
    pub references_current: f32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
#[allow(missing_docs)]
pub struct RefPrefix {
    pub value: String,
    pub name: String,
    pub public_references: bool,
    pub reference_visibility: Option<Visibility>,
}

/// response item for the `/journal/{id}` route
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[allow(missing_docs)]
pub struct Journal {
    /// could not determine type, possible PartialDateParts
    pub last_status_check_time: Option<Value>,
    pub counts: Option<usize>,
    pub breakdowns: Option<Value>,
    pub publisher: Option<String>,
    pub coverage: Option<Value>,
    pub title: Option<String>,
    pub subjects: Vec<Value>,
    pub coverage_type: Option<Value>,
    pub flags: Option<Value>,
    #[serde(rename = "ISSN")]
    pub issn: Vec<String>,
    pub issn_type: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;

    #[test]
    fn facets_deserialize() {
        let facets = r#"{
      "affiliation": {
        "value-count": 5,
        "values": {
          "of": 177247,
          "university": 147649,
          "department": 128741,
          "and": 102652,
          "medicine": 96232
        }
      },
      "orcid": {
        "value-count": 10,
        "values": {
          "http:\/\/orcid.org\/0000-0002-0270-1711": 67
        }
      }
    }"#;

        let _: FacetMap = from_str(facets).unwrap();
    }

    #[test]
    fn agency_msg_deserialize() {
        let agency_str =
            r#"{"status":"ok","message-type":"work-agency","message-version":"1.0.0","message":{"DOI":"10.1037\/0003-066x.59.1.29","agency":{"id":"crossref","label":"Crossref"}}}"#;

        let agency: Response = from_str(agency_str).unwrap();

        assert!(agency.is_work_ageny());
    }

    #[test]
    fn funder_list_msg_deserialize() {
        let funders_str = r#"{"status":"ok","message-type":"funder-list","message-version":"1.0.0","message":{"items-per-page":2,"query":{"start-index":0,"search-terms":"NSF"},"total-results":9,"items":[{ "id": "501100004190",
  "location": "Norway",
  "name": "Norsk  Sykepleierforbund",
  "alt-names": [
    "NSF"
  ],
  "uri": "http:\/\/dx.doi.org\/10.13039\/501100004190",
  "replaces": [],
  "replaced-by": [],
  "tokens": [
    "norsk"
  ]
}]}}"#;

        let funders: Response = from_str(funders_str).unwrap();

        assert!(funders.is_funder_list());
    }

    #[test]
    fn funder_msg_deserialize() {
        let funder_str = r#"{"status":"ok","message-type":"funder","message-version":"1.0.0","message":{ "id": "501100004190",
  "location": "Norway",
  "name": "Norsk  Sykepleierforbund",
  "alt-names": [
    "NSF"
  ],
  "uri": "http:\/\/dx.doi.org\/10.13039\/501100004190",
  "replaces": [],
  "replaced-by": [],
  "tokens": [
    "norsk"
  ],
  "work-count": 43,
  "descendants": [],
  "hierarchy-names": {
    "100000019": "National Hemophilia Foundation"
  },
  "descendant-work-count": 43,
    "hierarchy": {
      "100000019": {}
  }
}}"#;

        let funder: Response = from_str(funder_str).unwrap();

        assert!(funder.is_funder());
    }

    #[test]
    fn funder_msg_deserialize2() {
        let funder_str = r#"{"status":"ok","message-type":"funder","message-version":"1.0.0","message":{"hierarchy-names":{"100006130":"Office","100000015":"U.S. Department of Energy","100013165":"National"},"replaced-by":[],"work-count":44026,"name":"U.S. Department of Energy","descendants":["100006166"],"descendant-work-count":68704,"id":"100000015","tokens":["us"],"replaces":[],"uri":"http:\/\/dx.doi.org\/10.13039\/100000015","hierarchy":{"100000015":{"100006130":{"more":true},"100013165":{},"100006138":{"more":true}}},"alt-names":["DOE"],"location":"United States"}}"#;

        let funder: Response = from_str(funder_str).unwrap();

        assert!(funder.is_funder());
    }

    #[test]
    fn prefix_msg_deserialize() {
        let prefix_str = r#"{"status":"ok","message-type":"prefix","message-version":"1.0.0","message":{"member":"http:\/\/id.crossref.org\/member\/78","name":"Elsevier BV","prefix":"http:\/\/id.crossref.org\/prefix\/10.1016"}}"#;

        let prefix: Response = from_str(prefix_str).unwrap();

        assert!(prefix.is_prefix());
    }

    #[test]
    fn members_list_msg_deserialize() {
        let members_list_str = r#"{"status":"ok","message-type":"member-list","message-version":"1.0.0","message":{"items-per-page":2,"query":{"start-index":0,"search-terms":null},"total-results":10257,"items":[{"last-status-check-time":1551766727771,"primary-name":"Society for Leukocyte Biology","counts":{"total-dois":0,"current-dois":0,"backfile-dois":0},"breakdowns":{"dois-by-issued-year":[]},"prefixes":["10.1189"],"coverage":{"affiliations-current":0,"similarity-checking-current":0,"funders-backfile":0,"licenses-backfile":0,"funders-current":0,"affiliations-backfile":0,"resource-links-backfile":0,"orcids-backfile":0,"update-policies-current":0,"open-references-backfile":0,"orcids-current":0,"similarity-checking-backfile":0,"references-backfile":0,"award-numbers-backfile":0,"update-policies-backfile":0,"licenses-current":0,"award-numbers-current":0,"abstracts-backfile":0,"resource-links-current":0,"abstracts-current":0,"open-references-current":0,"references-current":0},"prefix":[{"value":"10.1189","name":"Society for Leukocyte Biology","public-references":false,"reference-visibility":"limited"}],"id":183,"tokens":["society","for","leukocyte","biology"],"counts-type":{"all":{},"current":{},"backfile":{}},"coverage-type":{"all":null,"backfile":null,"current":null},"flags":{"deposits-abstracts-current":false,"deposits-orcids-current":false,"deposits":false,"deposits-affiliations-backfile":false,"deposits-update-policies-backfile":false,"deposits-similarity-checking-backfile":false,"deposits-award-numbers-current":false,"deposits-resource-links-current":false,"deposits-articles":false,"deposits-affiliations-current":false,"deposits-funders-current":false,"deposits-references-backfile":false,"deposits-abstracts-backfile":false,"deposits-licenses-backfile":false,"deposits-award-numbers-backfile":false,"deposits-open-references-backfile":false,"deposits-open-references-current":false,"deposits-references-current":false,"deposits-resource-links-backfile":false,"deposits-orcids-backfile":false,"deposits-funders-backfile":false,"deposits-update-policies-current":false,"deposits-similarity-checking-current":false,"deposits-licenses-current":false},"location":"9650 Rockville Pike Attn: Lynn Willis Bethesda MD 20814 United States","names":["Society for Leukocyte Biology"]}]}}"#;

        let members_list: Response = from_str(members_list_str).unwrap();

        assert!(members_list.is_member_list());
    }

    #[test]
    fn member_msg_deserialize() {
        let member_str = r#"{"status":"ok","message-type":"member","message-version":"1.0.0","message":{"last-status-check-time":1551766727771,"primary-name":"Society for Leukocyte Biology","counts":{"total-dois":0,"current-dois":0,"backfile-dois":0},"breakdowns":{"dois-by-issued-year":[]},"prefixes":["10.1189"],"coverage":{"affiliations-current":0,"similarity-checking-current":0,"funders-backfile":0,"licenses-backfile":0,"funders-current":0,"affiliations-backfile":0,"resource-links-backfile":0,"orcids-backfile":0,"update-policies-current":0,"open-references-backfile":0,"orcids-current":0,"similarity-checking-backfile":0,"references-backfile":0,"award-numbers-backfile":0,"update-policies-backfile":0,"licenses-current":0,"award-numbers-current":0,"abstracts-backfile":0,"resource-links-current":0,"abstracts-current":0,"open-references-current":0,"references-current":0},"prefix":[{"value":"10.1189","name":"Society for Leukocyte Biology","public-references":false,"reference-visibility":"limited"}],"id":183,"tokens":["society","for","leukocyte","biology"],"counts-type":{"all":{},"current":{},"backfile":{}},"coverage-type":{"all":null,"backfile":null,"current":null},"flags":{"deposits-abstracts-current":false,"deposits-orcids-current":false,"deposits":false,"deposits-affiliations-backfile":false,"deposits-update-policies-backfile":false,"deposits-similarity-checking-backfile":false,"deposits-award-numbers-current":false,"deposits-resource-links-current":false,"deposits-articles":false,"deposits-affiliations-current":false,"deposits-funders-current":false,"deposits-references-backfile":false,"deposits-abstracts-backfile":false,"deposits-licenses-backfile":false,"deposits-award-numbers-backfile":false,"deposits-open-references-backfile":false,"deposits-open-references-current":false,"deposits-references-current":false,"deposits-resource-links-backfile":false,"deposits-orcids-backfile":false,"deposits-funders-backfile":false,"deposits-update-policies-current":false,"deposits-similarity-checking-current":false,"deposits-licenses-current":false},"location":"9650 Rockville Pike Attn: Lynn Willis Bethesda MD 20814 United States","names":["Society for Leukocyte Biology"]}}"#;

        let member: Response = from_str(member_str).unwrap();

        assert!(member.is_member());
    }

    #[test]
    fn journals_list_msg_deserialize() {
        let journal_list_str = r#"{"status":"ok","message-type":"journal-list","message-version":"1.0.0","message":{"items-per-page":2,"query":{"start-index":0,"search-terms":null},"total-results":10257,"items":[{"last-status-check-time":null,"counts":null,"breakdowns":null,"publisher":"Fundacao Educacional de Criciuma- FUCRI","coverage":null,"title":"A INFLU\u00caNCIA DA PUBLICIDADE NA TRANSI\u00c7\u00c3O NUTRICIONAL UMA S\u00cdNTESE PARA ENTENDER A OBESIDADE","subjects":[],"coverage-type":null,"flags":null,"ISSN":[],"issn-type":[]}]}}"#;

        let journal_list: Response = from_str(journal_list_str).unwrap();

        assert!(journal_list.is_journal_list());
    }

    #[test]
    fn journal_msg_deserialize() {
        let journal_str = r#"{"status":"ok","message-type":"journal","message-version":"1.0.0","message":{"last-status-check-time":null,"counts":null,"breakdowns":null,"publisher":"Fundacao Educacional de Criciuma- FUCRI","coverage":null,"title":"A INFLU\u00caNCIA DA PUBLICIDADE NA TRANSI\u00c7\u00c3O NUTRICIONAL UMA S\u00cdNTESE PARA ENTENDER A OBESIDADE","subjects":[],"coverage-type":null,"flags":null,"ISSN":[],"issn-type":[]}}"#;

        let journal: Response = from_str(journal_str).unwrap();

        assert!(journal.is_journal());
    }

    #[test]
    fn type_list_msg_deserialize() {
        let type_list_str = r#"{"status":"ok","message-type":"type-list","message-version":"1.0.0","message":{"total-results":27,"items":[{"id":"book-section","label":"Book Section"},{"id":"monograph","label":"Monograph"}]}}"#;
        let type_list: Response = from_str(type_list_str).unwrap();

        assert!(type_list.is_type_list());
    }

    #[test]
    fn type_msg_deserialize() {
        let type_str = r#"{"status":"ok","message-type":"type","message-version":"1.0.0","message":{"id":"book-section","label":"Book Section"}}"#;
        let type_: Response = from_str(type_str).unwrap();

        assert!(type_.is_type());
    }

    #[test]
    fn validation_failure_deserialize() {
        let failure_str = r#"{"status":"failed","message-type":"validation-failure","message":[{"type":"parameter-not-allowed","value":"query.*","message":"This route does not support field query parameters"}]}"#;
        let failure: Response = from_str(failure_str).unwrap();

        assert!(failure.is_validation_failure());
    }

    #[test]
    fn work_msg_deserialize() {
        let failure_str = r#"{"status":"ok","message-type":"work","message-version":"1.0.0","message":{"indexed":{"date-parts":[[2019,2,14]],"date-time":"2019-02-14T05:10:15Z","timestamp":1550121015066},"reference-count":105,"publisher":"American Psychological Association (APA)","issue":"1","content-domain":{"domain":[],"crossmark-restriction":false},"short-container-title":["American Psychologist"],"DOI":"10.1037\/0003-066x.59.1.29","type":"journal-article","created":{"date-parts":[[2004,1,21]],"date-time":"2004-01-21T14:31:19Z","timestamp":1074695479000},"page":"29-40","source":"Crossref","is-referenced-by-count":83,"title":["How the Mind Hurts and Heals the Body."],"prefix":"10.1037","volume":"59","author":[{"given":"Oakley","family":"Ray","sequence":"first","affiliation":[]}],"member":"15","published-online":{"date-parts":[[2004]]},"container-title":["American Psychologist"],"original-title":[],"language":"en","link":[{"URL":"http:\/\/psycnet.apa.org\/journals\/amp\/59\/1\/29.pdf","content-type":"unspecified","content-version":"vor","intended-application":"similarity-checking"}],"deposited":{"date-parts":[[2018,4,8]],"date-time":"2018-04-08T18:56:17Z","timestamp":1523213777000},"score":1.0,"subtitle":[],"short-title":[],"issued":{"date-parts":[[2004]]},"references-count":105,"journal-issue":{"published-online":{"date-parts":[[2004]]},"issue":"1"},"alternative-id":["2004-10043-004","14736318"],"URL":"http:\/\/dx.doi.org\/10.1037\/0003-066x.59.1.29","relation":{},"ISSN":["1935-990X","0003-066X"],"issn-type":[{"value":"0003-066X","type":"print"},{"value":"1935-990X","type":"electronic"}]}}"#;
        let work: Response = from_str(failure_str).unwrap();

        assert!(work.is_work());
    }
}
