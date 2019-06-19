use crate::error::{Error, ErrorKind, Result};
use crate::query::works::{WorksCombiner, WorksFilter, WorksIdentQuery, WorksQuery};
use crate::query::{Component, CrossrefQuery, CrossrefRoute, ResourceComponent};
use std::str::FromStr;

/// all possible types of a `Work`
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "id")]
#[serde(rename_all = "kebab-case")]
#[allow(missing_docs)]
pub enum Type {
    BookSection,
    Monograph,
    Report,
    PeerReview,
    BookTrack,
    JournalArticle,
    BookPart,
    Other,
    Book,
    JournalVolume,
    BookSet,
    ReferenceEntry,
    ProceedingsArticle,
    Journal,
    Component,
    BookChapter,
    ProceedingsSeries,
    ReportSeries,
    Proceedings,
    Standard,
    ReferenceBook,
    PostedContent,
    JournalIssue,
    Dissertation,
    Dataset,
    BookSeries,
    EditedBook,
    StandardSeries,
}

impl Type {
    /// the display-friendly label for the type
    pub fn label(&self) -> &str {
        match self {
            Type::BookSection => "Book Section",
            Type::Monograph => "Monograph",
            Type::Report => "Report",
            Type::PeerReview => "Peer Review",
            Type::BookTrack => "Book Track",
            Type::JournalArticle => "Journal Article",
            Type::BookPart => "Book Part",
            Type::Other => "Other",
            Type::Book => "Book",
            Type::JournalVolume => "Journal Volume",
            Type::BookSet => "Book Set",
            Type::ReferenceEntry => "Reference Entry",
            Type::ProceedingsArticle => "Proceedings Article",
            Type::Journal => "Journal",
            Type::Component => "Component",
            Type::BookChapter => "Book Chapter",
            Type::ProceedingsSeries => "Proceedings Series",
            Type::ReportSeries => "Report Series",
            Type::Proceedings => "Proceedings",
            Type::Standard => "Standard",
            Type::ReferenceBook => "Reference Book",
            Type::PostedContent => "Posted Content",
            Type::JournalIssue => "Journal Issue",
            Type::Dissertation => "Dissertation",
            Type::Dataset => "Dataset",
            Type::BookSeries => "Book Series",
            Type::EditedBook => "Edited Book",
            Type::StandardSeries => "Standard Series",
        }
    }
    /// the string used to identify the type
    pub fn id(&self) -> &str {
        match self {
            Type::BookSection => "book-section",
            Type::Monograph => "monograph",
            Type::Report => "report",
            Type::PeerReview => "peer-review",
            Type::BookTrack => "book-track",
            Type::JournalArticle => "journal-article",
            Type::BookPart => "book-part",
            Type::Other => "other",
            Type::Book => "book",
            Type::JournalVolume => "journal-volume",
            Type::BookSet => "book-set",
            Type::ReferenceEntry => "reference-entry",
            Type::ProceedingsArticle => "proceedings-article",
            Type::Journal => "journal",
            Type::Component => "component",
            Type::BookChapter => "book-chapter",
            Type::ProceedingsSeries => "proceedings-series",
            Type::ReportSeries => "report-series",
            Type::Proceedings => "proceedings",
            Type::Standard => "standard",
            Type::ReferenceBook => "reference-book",
            Type::PostedContent => "posted-content",
            Type::JournalIssue => "journal-issue",
            Type::Dissertation => "dissertation",
            Type::Dataset => "dataset",
            Type::BookSeries => "book-series",
            Type::EditedBook => "edited-book",
            Type::StandardSeries => "standard-series",
        }
    }
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "book-section" => Ok(Type::BookSection),
            "monograph" => Ok(Type::Monograph),
            "report" => Ok(Type::Report),
            "peer-review" => Ok(Type::PeerReview),
            "book-track" => Ok(Type::BookTrack),
            "journal-article" => Ok(Type::JournalArticle),
            "book-part" => Ok(Type::BookPart),
            "other" => Ok(Type::Other),
            "book" => Ok(Type::Book),
            "journal-volume" => Ok(Type::JournalVolume),
            "book-set" => Ok(Type::BookSet),
            "reference-entry" => Ok(Type::ReferenceEntry),
            "proceedings-article" => Ok(Type::ProceedingsArticle),
            "journal" => Ok(Type::Journal),
            "component" => Ok(Type::Component),
            "book-chapter" => Ok(Type::BookChapter),
            "proceedings-series" => Ok(Type::ProceedingsSeries),
            "report-series" => Ok(Type::ReportSeries),
            "proceedings" => Ok(Type::Proceedings),
            "standard" => Ok(Type::Standard),
            "reference-book" => Ok(Type::ReferenceBook),
            "posted-content" => Ok(Type::PostedContent),
            "journal-issue" => Ok(Type::JournalIssue),
            "dissertation" => Ok(Type::Dissertation),
            "dataset" => Ok(Type::Dataset),
            "book-series" => Ok(Type::BookSeries),
            "edited-book" => Ok(Type::EditedBook),
            "standard-series" => Ok(Type::StandardSeries),
            name => Err(Error::from(ErrorKind::InvalidTypeName {
                name: name.to_string(),
            })),
        }
    }
}

/// constructs the request payload for the `/types` route
#[derive(Debug, Clone)]
pub enum Types {
    /// every available type
    All,
    /// target a specific type at `/types/{id}`
    Identifier(String),
    /// target a `Work` for a specific type at `/types/{id}/works?query..`
    Works(WorksIdentQuery),
}

impl CrossrefRoute for Types {
    fn route(&self) -> Result<String> {
        match self {
            Types::All => Component::Types.route(),
            Types::Identifier(s) => Ok(format!("{}/{}", Component::Types.route()?, s)),
            Types::Works(combined) => Self::combined_route(combined),
        }
    }
}

impl CrossrefQuery for Types {
    fn resource_component(self) -> ResourceComponent {
        ResourceComponent::Types(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;

    //    #[test]
    fn test_types() {
        let section = r#"{
    "id": "book-section",
    "label": "Book Section"
  }"#;
        let ref_type: Type = serde_json::from_str(section).unwrap();

        assert_eq!(Type::BookSection, ref_type);
    }
}
