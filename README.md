Crossref-rs - A rust client for the Crossref-API
=====================
[![Build Status](https://travis-ci.com/MattsSe/crossref-rs.svg?branch=master)](https://travis-ci.com/MattsSe/crossref-rs)
[![Crates.io](https://img.shields.io/crates/v/crossref.svg)](https://crates.io/crates/crossref)
[![Documentation](https://docs.rs/crossref/badge.svg)](https://docs.rs/crossref)


[Crossref API docs](https://github.com/CrossRef/rest-api-doc>)

This client is inspired by [sckott/habanero](https://github.com/sckott/habanero/).


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

let query = WorksQuery::new("Machine Learning")
    // field queries supported for `Works`
    .field_query(FieldQuery::author("Some Author"))
    // filters are specific for each resource component
    .filter(WorksFilter::HasOrcid)
    .order(Order::Asc)
    .sort(Sort::Score);
```


### Get Records

See [this table](https://github.com/CrossRef/rest-api-doc#resource-components) for a detailed overview of the major components.

There are 3 different targets:

* **standalone resource components**: `/works`, `/members`, `funders`, `prefixes`, `types` that return a list list of the corresponding items and can be specified with queries
* **Resource component with identifiers**: `/works/{doi}?<query>`,`/members/{member_id}?<query>`, etc. that returns a single item if found.
* **combined with the `works` route**: The works component can be appended to other resources: `/members/{member_id}/works?<query>` etc. that returns a list of matching `Work` items as `WorkList`.

This resembles in the enums of the resource components, eg. for `Members`:

```rust
pub enum Members {
    /// target a specific member at `/members/{id}`
    Identifier(String),
    /// target all members that match the query at `/members?query...`
    Query(MembersQuery),
    /// target a `Work` for a specific member at `/members/{id}/works?query..`
    Works(WorksIdentQuery),
}
```

### Examples

All options are supported by the client:

**Query Single Item by DOI or ID**

Analogous methods exist for all resource components

```rust
let work = client.work("10.1037/0003-066X.59.1.29")?;

let agency = client.work_agency("10.1037/0003-066X.59.1.29")?;

let funder = client.funder("funder_id")?;

let member = client.member("member_id")?;
```

**Query**

```rust
let query = WorksQuery::new("Machine Learning");

// one page of the matching results
let works = client.works(query)?;
```

Alternatively insert a free form query term directly

```rust
let works = client.works("Machine Learning")?;
```

 **Combining Routes with the `Works` route**

For each resource component other than `Works` there exist methods to append a `WorksQuery` with the ID option `/members/{member_id}/works?<query>?`

```
use crossref::*;
fn run() -> Result<()> {
    let client = Crossref::builder().build()?;
    let works = client.member_works(WorksQuery::new("machine learning")
    .sort(Sort::Score).into_ident("member_id"))?;
    Ok(())
}
```

This would be the same as using the [`Crossref::works`] method by supplying the combined type

```rust
use crossref::*;
fn run() -> Result<()> {
    let client = Crossref::builder().build()?;
    let works = client.works(WorksQuery::new("machine learning")
     .sort(Sort::Score)
     .into_combined_query::<Members>("member_id"))?;
    Ok(())
}
```

** Deep paging for `Works` **
[Deep paging results](https://github.com/CrossRef/rest-api-doc#deep-paging-with-cursors)
Deep paging is supported for all queries, that return a list of `Work`, `WorkList`.
This function returns a new iterator over pages of `Work`, which is returned as bulk of items as a `WorkList` by crossref.
Usually a single page `WorkList` contains 20 items.

Example

Iterate over all `Works` linked to search term `Machine Learning`

```rust
use crossref::{Crossref, WorksQuery, Work};
fn run() -> Result<(), crossref::Error> {
    let client = Crossref::builder().build()?;
    
    let all_works: Vec<Work> = client.deep_page(WorksQuery::new("Machine Learning")).flat_map(|x|x.items).collect();
    
    Ok(())
}
```

Which can be simplified to
```rust
use crossref::{Crossref, WorksQuery, Work};
fn run() -> Result<(), crossref::Error> {
    let client = Crossref::builder().build()?;
    
    let all_works: Vec<Work> = client.deep_page("Machine Learning").into_work_iter().collect();
    
    Ok(())
}
```


Iterate over all the pages (`WorkList`) of the funder with id `funder id` by using a combined query.
A single `WorkList` usually holds 20 `Work` items.

```rust
use crossref::{Crossref, Funders, WorksQuery, Work, WorkList};
fn run() -> Result<(), crossref::Error> {
    let client = Crossref::builder().build()?;
    
    let all_funder_work_list: Vec<WorkList> = client.deep_page(WorksQuery::default()
            .into_combined_query::<Funders>("funder id")
      )
        .collect();
    
    Ok(())
}
```

Iterate over all `Work` items of a specfic funder directly.

```rust
use crossref::{Crossref, Funders, WorksQuery, Work, WorkList};
fn run() -> Result<(), crossref::Error> {
    let client = Crossref::builder().build()?;
    
    let all_works: Vec<Work> = client.deep_page(WorksQuery::default()
         .into_combined_query::<Funders>("funder id"))
         .into_work_iter()
         .collect();
    
    Ok(())
}
```


## Command Line Application

### Installation
```shell
cargo install crossref --features cli
```

### Usage

Top level subcommands
```text
USAGE:
    crossref <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    funders     Query crossref funders
    help        Prints this message or the help of the given subcommand(s)
    journals    Query crossref journals
    members     Query crossref members
    prefixes    Query crossref prefixes
    types       Query crossref types
    works       Query crossref works

```

Additional options for the component subcommands (querying, sorting, ordering and limiting is only supported for subcommands <works|funders|members> and is overridden by a present `--id` options)

```text
USAGE:
    crossref works [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -a, --append       if the output file already exists, append instead of overwriting the file
    -d, --deep-page    Enable deep paging. If a limit is set, then the limit takes priority.
    -h, --help         Prints help information
    -s, --silent       do not print anything
    -V, --version      Prints version information

OPTIONS:
    -i, --id <id>                    The id of component.
    -l, --limit <limit>              limit the amount of results
        --offset <offset>            Sets an offset where crossref begins to retrieve items.
        --order <order>              How to order the results: asc or desc
    -o <output>                      output path where the results shall be stored
        --polite <polite>            The email to use to get into crossref's polite pool
    -q, --query <query_terms>...     The free form terms for the query
        --sample <sample>            Request randoms Elements. Overrides all other options.
        --sort <sort>                How to sort the results, such as updated, indexed, published, issued
        --token <token>              The token to use for the crossref client
        --user-agent <user_agent>    The user agent to use for the crossref client
```

### Examples

Retrieve a specific work by a doi

```shell
 crossref works --id "10.1037/0003-066X.59.1.29"
```

Save the results as json

```shell
 crossref works --id "10.1037/0003-066X.59.1.29" -o output.json
```

Retrieve any other components by their ids

```shell
 crossref <works|journals|members|prefixes|types> --id "10.1037/0003-066X.59.1.29" -o output.json
```

Some components support additional filtering

```
crossref <works|funders|members> --query "A search term such as `Machine learning` for works" --limit 10 --offset 200 --order asc
```

Get `Works` of a specific component, such as a member with the id `98`:

```
crossref works member 98
```

`<prefix|funder|prefix|type` are also supported in the same way.


By default deep paging is disabled, hence the max amount of results of `Works` will be 20 (a single crossref page).
By enabling the `--deep-page` flag, all available results will be gathered.

To get in to the polite pool supply your email to the request headers with `--polite "polite@example.com"`

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
   
