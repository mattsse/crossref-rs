use crate::error::{Error, Result};
use crate::model::*;
use crate::query::facet::FacetCount;
use crate::query::funders::Funders;
use crate::query::journals::Journals;
use crate::query::member::Members;
use crate::query::prefixes::Prefixes;
use crate::query::types::{Type, Types};
use crate::query::works::{WorkFilter, Works};
use chrono::NaiveDate;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;

/// Helper trait for unified interface
pub trait CrossrefParams {
    /// the filter applied
    type Filter: Filter;
    /// all string queries
    fn query_terms(&self) -> &[String];
    /// the filters this object can use
    fn filters(&self) -> &[Self::Filter];
    /// the sort if set
    fn sort(&self) -> Option<&Sort>;
    /// the order if set
    fn order(&self) -> Option<&Order>;
    /// all facets this objects addresses
    fn facets(&self) -> &[FacetCount];
    /// the configured result control, if any
    fn result_control(&self) -> Option<&ResultControl>;
}

macro_rules! impl_common_query {
    ($i:ident, $filter:ident) => {
        /// Each query parameter is ANDed
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $i {
            /// search by non specific query
            pub queries: Vec<String>,
            /// filter to apply while querying
            pub filter: Vec<$filter>,
            /// sort results by a certain field and
            pub sort: Option<Sort>,
            /// set the sort order to `asc` or `desc`
            pub order: Option<Order>,
            /// enable facet information in responses
            pub facets: Vec<FacetCount>,
            /// deep page through `/works` result sets
            pub result_control: Option<ResultControl>,
        }

        impl $i {
            /// add a new free form query
            pub fn query(mut self, query: &str) -> Self {
                self.queries.push(query.to_string());
                self
            }
            /// add a new filter to the query
            pub fn filter(mut self, filter: $filter) -> Self {
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

            /// set result control option to query
            pub fn result_control(mut self, result_control: ResultControl) -> Self {
                self.result_control = Some(result_control);
                self
            }
        }

        impl CrossrefParams for $i {
            type Filter = $filter;

            fn query_terms(&self) -> &[String] {
                &self.queries
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
                self.result_control.as_ref()
            }
        }

        impl CrossrefRoute for $i {
            fn route(&self) -> Result<String> {
                let mut params = Vec::new();
                if !self.queries.is_empty() {
                    params.push(Cow::Owned(format!(
                        "query={}",
                        format_queries(&self.queries)
                    )));
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
                Ok(params.join("&"))
            }
        }
    };
}

pub mod facet;
pub mod funders;
pub mod journals;
pub mod member;
pub mod prefixes;
pub mod types;
pub mod works;

pub mod filter {
    pub use super::member::MembersFilter;
    pub use super::works::WorkFilter;
}

/// represents the visibility of an crossref item
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Visibility {
    Open,
    Limited,
    Closed,
}

impl Visibility {
    pub fn as_str(&self) -> &str {
        match self {
            Visibility::Open => "open",
            Visibility::Limited => "limited",
            Visibility::Closed => "closed",
        }
    }
}

/// Determines how results should be sorted
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum Order {
    /// list results in ascending order
    Asc,
    /// list results in descending order
    Desc,
}

impl Order {
    pub fn as_str(&self) -> &str {
        match self {
            Order::Asc => "asc",
            Order::Desc => "desc",
        }
    }
}

impl CrossrefQueryParam for Order {
    fn param_key(&self) -> Cow<str> {
        Cow::Borrowed("order")
    }

    fn param_value(&self) -> Option<Cow<str>> {
        Some(Cow::Borrowed(self.as_str()))
    }
}

/// Results from a list response can be sorted by applying the sort and order parameters.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum Sort {
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

impl Sort {
    pub fn as_str(&self) -> &str {
        match self {
            Sort::Score => "score",
            Sort::Updated => "updated",
            Sort::Deposited => "deposited",
            Sort::Indexed => "indexed",
            Sort::Published => "published",
            Sort::PublishedPrint => "published-print",
            Sort::PublishedOnline => "published-online",
            Sort::Issued => "issued",
            Sort::IsReferencedByCount => "is-reference-by-count",
            Sort::ReferenceCount => "reference-count",
        }
    }
}

impl CrossrefQueryParam for Sort {
    fn param_key(&self) -> Cow<str> {
        Cow::Borrowed("sort")
    }

    fn param_value(&self) -> Option<Cow<str>> {
        Some(Cow::Borrowed(self.as_str()))
    }
}

/// tells crossref how many items shall be returned or where to start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultControl {
    /// limits the returned items per page
    Rows(usize),
    /// sets an offset where crossref begins to retrieve items
    /// high offsets (~10k) result in long response times
    Offset(usize),
    /// combines rows and offset: limit returned items per page, starting at the offset
    RowsOffset { rows: usize, offset: usize },
    /// return random results
    Sample(usize),
}

impl CrossrefQueryParam for ResultControl {
    fn param_key(&self) -> Cow<str> {
        match self {
            ResultControl::Rows(_) => Cow::Borrowed("rows"),
            ResultControl::Offset(_) => Cow::Borrowed("offset"),
            ResultControl::RowsOffset { rows, .. } => Cow::Owned(format!("rows={}", rows)),
            ResultControl::Sample(_) => Cow::Borrowed("sample"),
        }
    }

    fn param_value(&self) -> Option<Cow<str>> {
        match self {
            ResultControl::Rows(r) | ResultControl::Offset(r) | ResultControl::Sample(r) => {
                Some(Cow::Owned(r.to_string()))
            }
            ResultControl::RowsOffset { offset, .. } => {
                Some(Cow::Owned(format!("offset={}", offset)))
            }
        }
    }
}

/// Major resource components supported by the Crossref API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Component {
    /// returns a list of all works (journal articles, conference proceedings, books, components, etc), 20 per page
    Works,
    /// returns a list of all funders in the [Funder Registry](https://github.com/Crossref/open-funder-registry)
    Funders,
    /// returns a list of all Crossref members (mostly publishers)
    Prefixes,
    /// returns a list of valid work types
    Members,
    /// return a list of licenses applied to works in Crossref metadata
    Types,
    /// return a list of journals in the Crossref database
    Journals,
}

impl Component {
    pub fn as_str(&self) -> &str {
        match self {
            Component::Works => "works",
            Component::Funders => "funders",
            Component::Prefixes => "prefixes",
            Component::Members => "members",
            Component::Types => "types",
            Component::Journals => "journals",
        }
    }
}

impl CrossrefRoute for Component {
    fn route(&self) -> Result<String> {
        Ok(format!("/{}", self.as_str()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceComponent {
    /// returns a list of all works (journal articles, conference proceedings, books, components, etc), 20 per page
    Works(Works),
    /// returns a list of all funders in the [Funder Registry](https://github.com/Crossref/open-funder-registry)
    Funders(Funders),
    /// returns a list of all Crossref members (mostly publishers)
    Prefixes(Prefixes),
    /// returns a list of valid work types
    Members(Members),
    /// return a list of licenses applied to works in Crossref metadata
    Types(Types),
    /// return a list of journals in the Crossref database
    Journals(Journals),
}

impl ResourceComponent {
    pub fn primary_component(&self) -> Component {
        match self {
            ResourceComponent::Works(_) => Component::Works,
            ResourceComponent::Funders(_) => Component::Funders,
            ResourceComponent::Prefixes(_) => Component::Prefixes,
            ResourceComponent::Members(_) => Component::Members,
            ResourceComponent::Types(_) => Component::Types,
            ResourceComponent::Journals(_) => Component::Journals,
        }
    }
}

impl CrossrefRoute for ResourceComponent {
    fn route(&self) -> Result<String> {
        match self {
            ResourceComponent::Works(c) => c.route(),
            ResourceComponent::Funders(c) => c.route(),
            ResourceComponent::Prefixes(c) => c.route(),
            ResourceComponent::Members(c) => c.route(),
            ResourceComponent::Types(c) => c.route(),
            ResourceComponent::Journals(c) => c.route(),
        }
    }
}

/// Helper trait to mark filters in the query string
pub trait Filter: ParamFragment {}

impl<T: Filter> CrossrefQueryParam for Vec<T> {
    /// always use `filter` as the key
    fn param_key(&self) -> Cow<str> {
        Cow::Borrowed("filter")
    }

    /// filters are multi value and values are concat with `,`
    fn param_value(&self) -> Option<Cow<str>> {
        Some(Cow::Owned(
            self.iter()
                .map(|x| x.fragment())
                .collect::<Vec<_>>()
                .join(","),
        ))
    }
}

/// represents a key value pair inside a multi value query string parameter
pub trait ParamFragment {
    /// the key, or name, of the fragmet
    fn key(&self) -> Cow<str>;

    /// the value of the fragment, if any
    fn value(&self) -> Option<Cow<str>>;

    /// key and value are concat using `:`
    fn fragment(&self) -> Cow<str> {
        if let Some(val) = self.value() {
            Cow::Owned(format!("{}:{}", self.key(), val))
        } else {
            self.key()
        }
    }
}

/// a trait used to capture parameters for the query string of the crossref api
pub trait CrossrefQueryParam {
    /// the key name of the parameter in the query string
    fn param_key(&self) -> Cow<str>;
    /// the value of the parameter, if any
    fn param_value(&self) -> Option<Cow<str>>;
    /// constructs the full parameter for the query string by combining the key and value
    fn param(&self) -> Cow<str> {
        if let Some(val) = self.param_value() {
            Cow::Owned(format!("{}={}", self.param_key(), val))
        } else {
            self.param_key()
        }
    }
}

impl<T: AsRef<str>> CrossrefQueryParam for (T, T) {
    fn param_key(&self) -> Cow<str> {
        Cow::Borrowed(self.0.as_ref())
    }

    fn param_value(&self) -> Option<Cow<str>> {
        Some(Cow::Borrowed(self.1.as_ref()))
    }
}

/// represents elements that constructs parts of the crossref request url
pub trait CrossrefRoute {
    /// constructs the route for the crossref api
    fn route(&self) -> Result<String>;
}

impl<T: CrossrefQueryParam> CrossrefRoute for AsRef<[T]> {
    fn route(&self) -> Result<String> {
        Ok(self
            .as_ref()
            .iter()
            .map(|x| x.param())
            .collect::<Vec<_>>()
            .join("&"))
    }
}

/// root level trait to construct full crossref api request urls
pub trait CrossrefQuery: CrossrefRoute {
    /// the resource component endpoint this route targets
    fn resource_component(&self) -> ResourceComponent;

    /// constructs the full request url by concating the [base_path] with the [route]
    fn to_url(&self, base_path: &str) -> Result<String> {
        Ok(format!("{}{}", base_path, self.route()?))
    }

    //    fn to_json(&self) -> Result<Value> {
    //        unimplemented!()
    //    }
}

/// formats the topic for crossref by replacing all whitespaces whit `+`
pub(crate) fn format_query<T: AsRef<str>>(topic: T) -> String {
    topic
        .as_ref()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("+")
}

/// formats the individual topics of a query into the format crossref expects
/// returns a single String consisting of all words combined by '+'
pub(crate) fn format_queries<T: AsRef<str>>(topics: &[T]) -> String {
    topics
        .iter()
        .map(format_query)
        .collect::<Vec<_>>()
        .join("+")
}
