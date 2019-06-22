use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
pub enum CnFormat {
    RdfXml,
    Turtle,
    CiteProcJson,
    CiteProcJsonIsh,
    Text,
    Ris,
    BibTex,
    CrossrefXml,
    DataciteXml,
    BibEntry,
    CrossrefTdm,
}

impl CnFormat {
    /// the mime identifier
    pub fn mime_type(&self) -> &str {
        match self {
            CnFormat::RdfXml => "text/xml",
            CnFormat::Turtle => "text/plain",
            CnFormat::CiteProcJson => "application/json",
            CnFormat::CiteProcJsonIsh => "application/json",
            CnFormat::Text => "text/plain",
            CnFormat::Ris => "text/plain",
            CnFormat::BibTex => "text/xml",
            CnFormat::CrossrefXml => "text/xml",
            CnFormat::DataciteXml => "",
            CnFormat::BibEntry => "text/plain",
            CnFormat::CrossrefTdm => "text/xml",
        }
    }
    /// the mime type's header
    pub fn header(&self) -> &str {
        match self {
            CnFormat::RdfXml => "application/rdf+xml",
            CnFormat::Turtle => "text/turtle",
            CnFormat::CiteProcJson | CnFormat::CiteProcJsonIsh => {
                "transform/application/vnd.citationstyles.csl+json"
            }
            CnFormat::Text => "text/x-bibliography",
            CnFormat::Ris => "application/x-research-info-systems",
            CnFormat::BibTex => "application/x-bibtex",
            CnFormat::CrossrefXml => "application/vnd.crossref.unixref+xml",
            CnFormat::DataciteXml => "application/vnd.datacite.datacite+xml",
            CnFormat::BibEntry => "application/x-bibtex",
            CnFormat::CrossrefTdm => "application/vnd.crossref.unixsd+xml",
        }
    }
}
