use crate::error::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct CrossRefType {
    /// Name of work's publisher
    pub id: String,
    /// Name of work's publisher
    pub label: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "id")]
#[serde(rename_all = "kebab-case")]
pub enum Types {
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

impl Types {
    /// the display-friendly label for the type
    fn label(&self) -> &str {
        match self {
            Types::BookSection => "Book Section",
            Types::Monograph => "Monograph",
            Types::Report => "Report",
            Types::PeerReview => "Peer Review",
            Types::BookTrack => "Book Track",
            Types::JournalArticle => "Journal Article",
            Types::BookPart => "Book Part",
            Types::Other => "Other",
            Types::Book => "Book",
            Types::JournalVolume => "Journal Volume",
            Types::BookSet => "Book Set",
            Types::ReferenceEntry => "Reference Entry",
            Types::ProceedingsArticle => "Proceedings Article",
            Types::Journal => "Journal",
            Types::Component => "Component",
            Types::BookChapter => "Book Chapter",
            Types::ProceedingsSeries => "Proceedings Series",
            Types::ReportSeries => "Report Series",
            Types::Proceedings => "Proceedings",
            Types::Standard => "Standard",
            Types::ReferenceBook => "Reference Book",
            Types::PostedContent => "Posted Content",
            Types::JournalIssue => "Journal Issue",
            Types::Dissertation => "Dissertation",
            Types::Dataset => "Dataset",
            Types::BookSeries => "Book Series",
            Types::EditedBook => "Edited Book",
            Types::StandardSeries => "Standard Series",
        }
    }
    /// the string used to identify the type
    fn id(&self) -> &str {
        match self {
            Types::BookSection => "book-section",
            Types::Monograph => "monograph",
            Types::Report => "report",
            Types::PeerReview => "peer-review",
            Types::BookTrack => "book-track",
            Types::JournalArticle => "journal-article",
            Types::BookPart => "book-part",
            Types::Other => "other",
            Types::Book => "book",
            Types::JournalVolume => "journal-volume",
            Types::BookSet => "book-set",
            Types::ReferenceEntry => "reference-entry",
            Types::ProceedingsArticle => "proceedings-article",
            Types::Journal => "journal",
            Types::Component => "component",
            Types::BookChapter => "book-chapter",
            Types::ProceedingsSeries => "proceedings-series",
            Types::ReportSeries => "report-series",
            Types::Proceedings => "proceedings",
            Types::Standard => "standard",
            Types::ReferenceBook => "reference-book",
            Types::PostedContent => "posted-content",
            Types::JournalIssue => "journal-issue",
            Types::Dissertation => "dissertation",
            Types::Dataset => "dataset",
            Types::BookSeries => "book-series",
            Types::EditedBook => "edited-book",
            Types::StandardSeries => "standard-series",
        }
    }
}

impl FromStr for Types {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "book-section" => Ok(Types::BookSection),
            "monograph" => Ok(Types::Monograph),
            "report" => Ok(Types::Report),
            "peer-review" => Ok(Types::PeerReview),
            "book-track" => Ok(Types::BookTrack),
            "journal-article" => Ok(Types::JournalArticle),
            "book-part" => Ok(Types::BookPart),
            "other" => Ok(Types::Other),
            "book" => Ok(Types::Book),
            "journal-volume" => Ok(Types::JournalVolume),
            "book-set" => Ok(Types::BookSet),
            "reference-entry" => Ok(Types::ReferenceEntry),
            "proceedings-article" => Ok(Types::ProceedingsArticle),
            "journal" => Ok(Types::Journal),
            "component" => Ok(Types::Component),
            "book-chapter" => Ok(Types::BookChapter),
            "proceedings-series" => Ok(Types::ProceedingsSeries),
            "report-series" => Ok(Types::ReportSeries),
            "proceedings" => Ok(Types::Proceedings),
            "standard" => Ok(Types::Standard),
            "reference-book" => Ok(Types::ReferenceBook),
            "posted-content" => Ok(Types::PostedContent),
            "journal-issue" => Ok(Types::JournalIssue),
            "dissertation" => Ok(Types::Dissertation),
            "dataset" => Ok(Types::Dataset),
            "book-series" => Ok(Types::BookSeries),
            "edited-book" => Ok(Types::EditedBook),
            "standard-series" => Ok(Types::StandardSeries),
            name => Err(Error::InvalidTypeName {
                name: name.to_string(),
            }),
        }
    }
}

impl Into<CrossRefType> for Types {
    fn into(self) -> CrossRefType {
        CrossRefType {
            id: self.id().to_string(),
            label: self.label().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;

    #[test]
    fn test_types() {
        let section = r#"{
    "id": "book-section",
    "label": "Book Section"
  }"#;
        let ref_type: Types = serde_json::from_str(section).unwrap();

        assert_eq!(Types::BookSection, ref_type);
    }
}
