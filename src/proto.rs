use crate::model::Work;

/// Represents the crossref response for a `work` request.
/// requesting an url: `https://api.crossref.org/works/10.1037/0003-066X.59.1.29/agency`
/// will return following result:
/// rust-norun```{
///  status: "ok",
///  message-type: "work-agency",
///  message-version: "1.0.0",
///  message: {
///    DOI: "10.1037/0003-066x.59.1.29",
///    agency: {
///      id: "crossref",
///      label: "Crossref"
///    }
///  }```
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
    // the actual message of the response
    //    pub message: Option<Work>,
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
#[serde(untagged)]
// TODO [MessageFormat] sufficient?
pub enum MessageType {
    Funder,
    Prefix,
    Member,
    Work,
    WorkList,
    FunderList,
    PrefixList,
    MemberList,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Item {
    Agency,
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
}
