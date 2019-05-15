Crossref-rs - A rust client for the Crossref-API
=====================
[![Build Status](https://travis-ci.com/MattsSe/crossref-rs.svg?branch=master)](https://travis-ci.com/MattsSe/crossref-rs)
[![Crates.io](https://img.shields.io/crates/v/crossref.svg)](https://crates.io/crates/crossref)
[![Documentation](https://docs.rs/crossref/badge.svg)](https://docs.rs/crossref)


[Crossref API docs](https://github.com/CrossRef/rest-api-doc>)

This client is inspired by [sckott/habanero](https://github.com/sckott/habanero/).

This is still WIP.

`Crossref` - Crossref search API. The `Crossref` crate provides methods matching Crossref API routes:

* `works` - `/works` route
* `members` - `/members` route
* `prefixes` - `/prefixes` route
* `funders` - `/funders` route
* `journals` - `/journals` route
* `types` - `/types` route
* `agency` - `/works/{doi}/agency` get DOI minting agency


## Usage

### Create a `Crossref` client:

```rust
let client = Crossref::builder().build()?;
```

If you have an [Authorization token for Crossref's Plus service](https://github.com/CrossRef/rest-api-doc#authorization-token-for-plus-service):

```rust
let client = Crossref::builder()
    .token("token")
    .build()?;
```

Encouraged to use the **The Polite Pool**:

[Good manners = more reliable service](https://github.com/CrossRef/rest-api-doc#good-manners--more-reliable-service)

To get into Crossref's polite pool include a email address

```rust
let client = Crossref::builder()
     .polite("polite@example.com")
     .token("your token")
     .build()?;
```

### Constructing Queries
Not all components support queries and there are custom available parameters for each route that supports querying.
For each resource components that supports querying there exist a Query struct: `WorksQuery`, `MembersQuery`, `FundersQuery`. The `WorksQuery` also differs from the others by supporting [deep paging with cursors](https://github.com/CrossRef/rest-api-doc#deep-paging-with-cursors) and [field queries](https://github.com/CrossRef/rest-api-doc#works-field-queries). 

otherwise creating queries works the same for all resource components:

```rust

let query = WorksQuery::new()
    .query("Machine Learning")
    // field queries supported for `Works`
    .field_query(FieldQuery::author("Some Author"))
    // filters are specific for each resource component
    .filter(WorksFilter::HasOrcid)
    .order(Order::Asc)
    .sort(Sort::Score);
```


### Get Records

See [this table](https://github.com/CrossRef/rest-api-doc#resource-components) for a detailed overview of the major components.

There are 3 available targets:

* **standalone resource components**: `/works`, `/members`, etc. that return a list list of the corresponding items and can be specified with queries
* **Resource component with identifiers**: `/works/{doi}?<query>`,`/members/{member_id}?<query>`, etc. that returns a single item if found.
* **combined with the `works` route**: The works component can be appended to other resources: `/members/{member_id}/works?<query>` etc. that returns a list of matching `Work` items.

This resembles in the enums of the resource components, eg. for `Members`:

```rust
pub enum Members {
    /// target a specific member at `/members/{id}`
    Identifier(String),
    /// target all members that match the query at `/members?query...`
    Query(MembersQuery),
    /// target a `Work` for a specific member at `/members/{id}/works?query..`
    Works(WorksCombined),
}
```


All options are supported by the client:

**Single Item by DOI (ID)**

Analogous methods exist for all resource components

```rust
let work = client.work("10.1037/0003-066X.59.1.29")?;

let agency = client.work_agency("10.1037/0003-066X.59.1.29")?;

let funder = client.funder("funder_id")?;

let member = client.member("member_id")?;
```

**Query**

```rust
let query = WorksQuery::new().query("Machine Learning");

// one page of the matching results
let works = client.works(query)?;
```

**Convenience method for Work Items by terms**

```rust
let works = client.query_works("Machine Learning")?;
```

**Combining Routes with the `Works` route**

For each resource component other than `Works` there exist methods to append a `WorksQuery` with the ID option `/members/{member_id}/works?<query>?`

```rust
let works = client.member_works("member_id", WorksQuery::new()
    .query("machine learning")
    .sort(Sort::Score))?;
```

Convenience method to append works query term:

```rust

let works = client.member_works("member id", "Machine Learning")?;
``` 

**Deep paging for Works**


## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
   
