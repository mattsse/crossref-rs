use crate::model::Work;
use crate::model::*;
use crate::query::facet::Facet;
use crate::query::facet::FacetCount;
use crate::query::Visibility;
use serde_json::Value;
use std::collections::HashMap;

/// Represents the crossref response for a `work` request.
/// requesting an url: `https://api.crossref.org/works/10.1037/0003-066X.59.1.29/agency`
/// will return following result:
/// r#"{
///  status: "ok",
///  message-type: "work-agency",
///  message-version: "1.0.0",
///  message: {
///    DOI: "10.1037/0003-066x.59.1.29",
///    agency: {
///      id: "crossref",
///      label: "Crossref"
///    }
///  }#"
///
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CrossrefResponse {
    /// the status of the request
    pub status: String,
    /// the type of the response message holds
    pub message_type: MessageType,
    /// the version of the service created this message
    pub message_version: String,
    /// the actual message of the response
    pub message: Option<Msg>,

    /// the number of elements to expect
    pub items_per_page: usize,

    /// information about the submitted query
    pub query: Option<QueryResponse>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Msg {
    #[serde(default)]
    pub facets: FacetMap,
    pub total_results: usize,
    pub items: Vec<ResponseMessage>,
    pub items_per_page: usize,
    pub query: Option<QueryResponse>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Message {
    /// Singletons are single results. Retrieving metadata for a specific identifier
    /// (e.g. DOI, ISSN, funder_identifier) typically returns in a singleton result.
    Singleton {},
    Multi {
        total_results: i32,
        items_per_page: i32,
        items: Vec<Work>,
    },
    //    HeadersOnly
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MessageFormat {
    Single(String),
    List(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MessageType {
    WorkAgency,
    Funder,
    Prefix,
    Member,
    Work,
    WorkList,
    FunderList,
    PrefixList,
    MemberList,
}

// TODO impl custom map deserializer https://serde.rs/deserialize-map.html
pub type FacetMap = HashMap<String, FacetItem>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FacetItem {
    /// represents the length of `values`
    pub value_count: usize,
    /// contains the
    pub values: HashMap<String, usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResponseMessage {
    Agency {
        #[serde(rename = "DOI")]
        doi: String,
        agency: Agency,
    },
    Prefix {
        member: String,
        name: String,
        prefix: String,
    },
    Funder(Box<Funder>),
    FunderList(Vec<Funder>),
    Work(Box<Work>),
    WorkList(Vec<Work>),
    MemberList(Vec<Member>),
    Member(Box<Member>),
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Funder {
    pub hierarchy_names: HashMap<String, Option<String>>,
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
    pub hierarchy: HashMap<String, HashMap<String, HashMap<String, bool>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Member {
    pub last_status_check_time: usize,
    pub primary_name: String,
    pub counts: Counts,
    pub breakdowns: Breakdowns,
    pub prefixes: Vec<String>,
    pub coverage: Coverage,
    pub prefix: Vec<Prefix>,
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
pub struct Counts {
    pub total_dois: usize,
    pub current_dois: usize,
    pub backfile_dois: usize,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Breakdowns {
    pub dois_by_issued_year: Vec<Vec<u32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
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

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Prefix {
    pub value: String,
    pub name: String,
    pub public_references: bool,
    pub reference_visibility: Option<Visibility>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;
    //    #[test]
    fn serde_response() {
        let section = r#"{
  "status": "ok",
  "message-type": "work-agency",
  "message-version": "1.0.0"
}"#;
        let ref_type: CrossrefResponse = serde_json::from_str(section).unwrap();

        //        assert_eq!(
        //            MessageFormat::Single("work-agency".to_string()),
        //            ref_type.message_type
        //        )
    }

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
            r#"{"DOI":"10.1037\/0003-066x.59.1.29","agency":{"id":"crossref","label":"Crossref"}}"#;

        let agency: ResponseMessage = from_str(agency_str).unwrap();

        match agency {
            ResponseMessage::Agency { .. } => {}
            _ => panic!("expected Agency"),
        }
    }

    #[test]
    fn funder_list_msg_deserialize() {
        let funders_str = r#"[{ "id": "501100004190",
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
}]"#;

        let funders: ResponseMessage = from_str(funders_str).unwrap();
        match funders {
            ResponseMessage::FunderList(_) => {}
            _ => panic!("expected FunderList"),
        }
    }
    #[test]
    fn funder_msg_deserialize() {
        let funder_str = r#"{ "id": "501100004190",
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
}"#;

        let funder: ResponseMessage = from_str(funder_str).unwrap();
        match funder {
            ResponseMessage::Funder(_) => {}
            _ => panic!("expected single Funder"),
        }
    }

    #[test]
    fn funder_msg_deserialize2() {
        let funder_str = r#"{"hierarchy-names":{"100006130":"Office","100000015":"U.S. Department of Energy","100013165":"National"},"replaced-by":[],"work-count":44026,"name":"U.S. Department of Energy","descendants":["100006166"],"descendant-work-count":68704,"id":"100000015","tokens":["us"],"replaces":[],"uri":"http:\/\/dx.doi.org\/10.13039\/100000015","hierarchy":{"100000015":{"100006130":{"more":true},"100013165":{},"100006138":{"more":true}}},"alt-names":["DOE"],"location":"United States"}"#;

        let funder: ResponseMessage = from_str(funder_str).unwrap();
        match funder {
            ResponseMessage::Funder(_) => {}
            _ => panic!("expected single Funder"),
        }
    }

    #[test]
    fn prefix_msg_deserialize() {
        let prefix_str = r#"{"member":"http:\/\/id.crossref.org\/member\/78","name":"Elsevier BV","prefix":"http:\/\/id.crossref.org\/prefix\/10.1016"}"#;

        let prefix: ResponseMessage = from_str(prefix_str).unwrap();
        match prefix {
            ResponseMessage::Prefix { .. } => {}
            _ => panic!("expected Prefix"),
        }
    }
    #[test]
    fn members_list_msg_deserialize() {
        let members_list_str = r#"[{"last-status-check-time":1551766727771,"primary-name":"Society for Leukocyte Biology","counts":{"total-dois":0,"current-dois":0,"backfile-dois":0},"breakdowns":{"dois-by-issued-year":[]},"prefixes":["10.1189"],"coverage":{"affiliations-current":0,"similarity-checking-current":0,"funders-backfile":0,"licenses-backfile":0,"funders-current":0,"affiliations-backfile":0,"resource-links-backfile":0,"orcids-backfile":0,"update-policies-current":0,"open-references-backfile":0,"orcids-current":0,"similarity-checking-backfile":0,"references-backfile":0,"award-numbers-backfile":0,"update-policies-backfile":0,"licenses-current":0,"award-numbers-current":0,"abstracts-backfile":0,"resource-links-current":0,"abstracts-current":0,"open-references-current":0,"references-current":0},"prefix":[{"value":"10.1189","name":"Society for Leukocyte Biology","public-references":false,"reference-visibility":"limited"}],"id":183,"tokens":["society","for","leukocyte","biology"],"counts-type":{"all":{},"current":{},"backfile":{}},"coverage-type":{"all":null,"backfile":null,"current":null},"flags":{"deposits-abstracts-current":false,"deposits-orcids-current":false,"deposits":false,"deposits-affiliations-backfile":false,"deposits-update-policies-backfile":false,"deposits-similarity-checking-backfile":false,"deposits-award-numbers-current":false,"deposits-resource-links-current":false,"deposits-articles":false,"deposits-affiliations-current":false,"deposits-funders-current":false,"deposits-references-backfile":false,"deposits-abstracts-backfile":false,"deposits-licenses-backfile":false,"deposits-award-numbers-backfile":false,"deposits-open-references-backfile":false,"deposits-open-references-current":false,"deposits-references-current":false,"deposits-resource-links-backfile":false,"deposits-orcids-backfile":false,"deposits-funders-backfile":false,"deposits-update-policies-current":false,"deposits-similarity-checking-current":false,"deposits-licenses-current":false},"location":"9650 Rockville Pike Attn: Lynn Willis Bethesda MD 20814 United States","names":["Society for Leukocyte Biology"]}]"#;

        let members_list: ResponseMessage = from_str(members_list_str).unwrap();
        match members_list {
            ResponseMessage::MemberList(_) => {}
            _ => panic!("expected MemberList"),
        }
    }
    #[test]
    fn member_msg_deserialize() {
        let member_str = r#"{"last-status-check-time":1551766727771,"primary-name":"Society for Leukocyte Biology","counts":{"total-dois":0,"current-dois":0,"backfile-dois":0},"breakdowns":{"dois-by-issued-year":[]},"prefixes":["10.1189"],"coverage":{"affiliations-current":0,"similarity-checking-current":0,"funders-backfile":0,"licenses-backfile":0,"funders-current":0,"affiliations-backfile":0,"resource-links-backfile":0,"orcids-backfile":0,"update-policies-current":0,"open-references-backfile":0,"orcids-current":0,"similarity-checking-backfile":0,"references-backfile":0,"award-numbers-backfile":0,"update-policies-backfile":0,"licenses-current":0,"award-numbers-current":0,"abstracts-backfile":0,"resource-links-current":0,"abstracts-current":0,"open-references-current":0,"references-current":0},"prefix":[{"value":"10.1189","name":"Society for Leukocyte Biology","public-references":false,"reference-visibility":"limited"}],"id":183,"tokens":["society","for","leukocyte","biology"],"counts-type":{"all":{},"current":{},"backfile":{}},"coverage-type":{"all":null,"backfile":null,"current":null},"flags":{"deposits-abstracts-current":false,"deposits-orcids-current":false,"deposits":false,"deposits-affiliations-backfile":false,"deposits-update-policies-backfile":false,"deposits-similarity-checking-backfile":false,"deposits-award-numbers-current":false,"deposits-resource-links-current":false,"deposits-articles":false,"deposits-affiliations-current":false,"deposits-funders-current":false,"deposits-references-backfile":false,"deposits-abstracts-backfile":false,"deposits-licenses-backfile":false,"deposits-award-numbers-backfile":false,"deposits-open-references-backfile":false,"deposits-open-references-current":false,"deposits-references-current":false,"deposits-resource-links-backfile":false,"deposits-orcids-backfile":false,"deposits-funders-backfile":false,"deposits-update-policies-current":false,"deposits-similarity-checking-current":false,"deposits-licenses-current":false},"location":"9650 Rockville Pike Attn: Lynn Willis Bethesda MD 20814 United States","names":["Society for Leukocyte Biology"]}"#;

        let member: ResponseMessage = from_str(member_str).unwrap();
        match member {
            ResponseMessage::Member(_) => {}
            _ => panic!("expected Member"),
        }
    }
}
