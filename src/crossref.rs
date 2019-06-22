use crossref::{query::*, Crossref, Order, Sort, WorkResultControl, WorksQuery};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

macro_rules! query {
    ($query:ident, $opts:ident) => {
        $query.queries = $opts.query_terms.clone();
        $query.sort = $opts.sort.clone();
        $query.order = $opts.order.clone();
        if let Some(offset) = $opts.offset {
            if let Some(rows) = $opts.limit {
                $query.result_control = Some(ResultControl::RowsOffset { rows, offset })
            } else {
                $query.result_control = Some(ResultControl::Offset(offset))
            }
        }
        if let Some(limit) = $opts.limit {
            $query.result_control = Some(ResultControl::Rows(limit))
        }
        if let Some(sample) = $opts.sample {
            $query.result_control = Some(ResultControl::Sample(sample))
        }
    };
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "crossref",
    about = "Access the crossref API from the command line."
)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
enum App {
    #[structopt(name = "works", about = "Query crossref works")]
    Works {
        #[structopt(
            short = "d",
            long = "deep-page",
            help = "Enable deep paging. If a limit is set, then the limit takes priority."
        )]
        deep_page: bool,
        #[structopt(flatten)]
        opts: Opts,
        #[structopt(subcommand)]
        combined: Option<Combined>,
    },
    #[structopt(name = "funders", about = "Query crossref funders")]
    Funders {
        #[structopt(flatten)]
        opts: Opts,
    },
    #[structopt(name = "members", about = "Query crossref members")]
    Members {
        #[structopt(flatten)]
        opts: Opts,
    },
    #[structopt(name = "journals", about = "Query crossref journals")]
    Journals {
        #[structopt(long = "id", help = "The id of component.")]
        id: String,
        #[structopt(flatten)]
        client_opts: ClientOpts,
        #[structopt(flatten)]
        out: Out,
    },
    #[structopt(name = "prefixes", about = "Query crossref prefixes")]
    Prefixes {
        #[structopt(long = "id", help = "The id of component.")]
        id: String,
        #[structopt(flatten)]
        client_opts: ClientOpts,
        #[structopt(flatten)]
        out: Out,
    },
    #[structopt(name = "types", about = "Query crossref types")]
    Types {
        #[structopt(parse(try_from_str), long = "id", help = "The id of component.")]
        id: Option<Type>,
        #[structopt(flatten)]
        client_opts: ClientOpts,
        #[structopt(flatten)]
        out: Out,
    },
}

impl App {
    pub fn client_opts(&self) -> &ClientOpts {
        match self {
            App::Works { opts, .. } | App::Funders { opts, .. } | App::Members { opts, .. } => {
                &opts.client_opts
            }

            App::Prefixes { client_opts, .. }
            | App::Types { client_opts, .. }
            | App::Journals { client_opts, .. } => client_opts,
        }
    }

    pub fn out(&self) -> &Out {
        match self {
            App::Works { opts, .. } | App::Funders { opts, .. } | App::Members { opts, .. } => {
                &opts.out
            }

            App::Prefixes { out, .. } | App::Types { out, .. } | App::Journals { out, .. } => out,
        }
    }

    pub fn get_value<W>(&self, writer: W, client: &Crossref) -> crossref::Result<()>
    where
        W: std::io::Write,
    {
        match self {
            App::Types { id, .. } => {
                if let Some(id) = id {
                    Ok(serde_json::to_writer_pretty(writer, &client.type_(id)?)?)
                } else {
                    Ok(serde_json::to_writer_pretty(writer, &client.types()?)?)
                }
            }
            App::Prefixes { id, .. } => Ok(serde_json::to_writer_pretty(
                writer,
                &client.prefix(id.as_str())?,
            )?),
            App::Journals { id, .. } => Ok(serde_json::to_writer_pretty(
                writer,
                &client.journal(id.as_str())?,
            )?),
            App::Members { opts, .. } => {
                if let Some(id) = &opts.id {
                    Ok(serde_json::to_writer_pretty(
                        writer,
                        &client.member(id.as_str())?,
                    )?)
                } else {
                    let mut query = MembersQuery::default();
                    query!(query, opts);
                    Ok(serde_json::to_writer_pretty(
                        writer,
                        &client.members(query)?,
                    )?)
                }
            }
            App::Funders { opts, .. } => {
                if let Some(id) = &opts.id {
                    Ok(serde_json::to_writer_pretty(
                        writer,
                        &client.funder(id.as_str())?,
                    )?)
                } else {
                    let mut query = FundersQuery::default();
                    query!(query, opts);
                    Ok(serde_json::to_writer_pretty(
                        writer,
                        &client.funders(query)?,
                    )?)
                }
            }
            App::Works {
                opts,
                combined,
                deep_page,
            } => {
                if let Some(id) = &opts.id {
                    Ok(serde_json::to_writer_pretty(
                        writer,
                        &client.work(id.as_str())?,
                    )?)
                } else {
                    let mut query = WorksQuery::default();
                    query.free_form_queries = opts.query_terms.clone();
                    query.sort = opts.sort.clone();
                    query.order = opts.order.clone();
                    if let Some(offset) = opts.offset {
                        if let Some(rows) = opts.limit {
                            query.result_control =
                                Some(WorkResultControl::Standard(ResultControl::RowsOffset {
                                    rows,
                                    offset,
                                }))
                        } else {
                            query.result_control =
                                Some(WorkResultControl::Standard(ResultControl::Offset(offset)))
                        }
                    }
                    if let Some(limit) = opts.limit {
                        query.result_control =
                            Some(WorkResultControl::Standard(ResultControl::Rows(limit)))
                    }
                    if let Some(sample) = opts.sample {
                        query.result_control =
                            Some(WorkResultControl::Standard(ResultControl::Sample(sample)))
                    }

                    if let Some(combined) = combined {
                        let query = match combined {
                            Combined::Journal { id, .. } => {
                                query.into_combined_query::<Journals>(id.as_str())
                            }
                            Combined::Type { id, .. } => {
                                query.into_combined_query::<Types>(id.as_str())
                            }
                            Combined::Funder { id, .. } => {
                                query.into_combined_query::<Funders>(id.as_str())
                            }
                            Combined::Member { id, .. } => {
                                query.into_combined_query::<Members>(id.as_str())
                            }
                            Combined::Prefix { id, .. } => {
                                query.into_combined_query::<Prefixes>(id.as_str())
                            }
                        };

                        if *deep_page {
                            Ok(serde_json::to_writer_pretty(
                                writer,
                                &client.deep_page(query).into_work_iter().collect::<Vec<_>>(),
                            )?)
                        } else {
                            Ok(serde_json::to_writer_pretty(writer, &client.works(query)?)?)
                        }
                    } else {
                        Ok(serde_json::to_writer_pretty(writer, &client.works(query)?)?)
                    }
                }
            }
        }
    }
}

#[derive(Debug, StructOpt)]
enum Combined {
    #[structopt(name = "member", about = "Get Works of a specific Member")]
    Member { id: String },
    #[structopt(name = "funder", about = "Get Works of a specific Funder")]
    Funder { id: String },
    #[structopt(name = "journal", about = "Get Works of a specific Journal")]
    Journal { id: String },
    #[structopt(name = "prefix", about = "Get Works of a specific Prefix")]
    Prefix { id: String },
    #[structopt(name = "type", about = "Get Works of a specific Type")]
    Type { id: String },
}

#[derive(Debug, StructOpt)]
struct Out {
    #[structopt(
        short = "o",
        parse(from_os_str),
        help = "output path where the results shall be stored"
    )]
    output: Option<PathBuf>,
    #[structopt(
        short = "a",
        long = "append",
        help = "if the output file already exists, append instead of overwriting the file"
    )]
    append: bool,
    #[structopt(short = "s", long = "silent", help = "do not print anything")]
    silent: bool,
}

#[derive(Debug, StructOpt)]
struct ClientOpts {
    #[structopt(
        long = "user-agent",
        help = "The user agent to use for the crossref client"
    )]
    user_agent: Option<String>,
    #[structopt(long = "token", help = "The token to use for the crossref client")]
    token: Option<String>,
    #[structopt(
        long = "polite",
        help = "The email to use to get into crossref's polite pool"
    )]
    polite: Option<String>,
}

impl ClientOpts {
    pub fn create_client(&self) -> Result<Crossref, crossref::Error> {
        let mut builder = Crossref::builder();

        if let Some(agent) = &self.user_agent {
            builder = builder.user_agent(agent.as_str());
        }
        if let Some(token) = &self.token {
            builder = builder.token(token.as_str());
        }
        if let Some(polite) = &self.polite {
            builder = builder.polite(polite.as_str());
        }
        builder.build()
    }
}

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(flatten)]
    out: Out,
    #[structopt(short = "l", long = "limit", help = "limit the amount of results")]
    limit: Option<usize>,

    #[structopt(short = "i", long = "id", help = "The id of component.")]
    id: Option<String>,

    #[structopt(
        short = "q",
        long = "query",
        help = "The free form terms for the query"
    )]
    query_terms: Vec<String>,

    #[structopt(
        long = "sort",
        help = "How to sort the results, such as updated, indexed, published, issued"
    )]
    sort: Option<Sort>,

    #[structopt(long = "order", help = "How to order the results: asc or desc")]
    order: Option<Order>,
    #[structopt(
        long = "sample",
        help = "Request randoms Elements. Overrides all other options."
    )]
    sample: Option<usize>,
    #[structopt(
        long = "offset",
        help = "Sets an offset where crossref begins to retrieve items."
    )]
    offset: Option<usize>,

    #[structopt(flatten)]
    client_opts: ClientOpts,
}

fn main() -> Result<(), failure::Error> {
    pretty_env_logger::try_init()?;
    let app = App::from_args();

    let client = app.client_opts().create_client()?;

    let out = app.out();
    if let Some(path) = &out.output {
        let file = if out.append && path.exists() {
            fs::OpenOptions::new().write(true).append(true).open(path)?
        } else {
            fs::File::create(path)?
        };
        app.get_value(file, &client)?
    } else {
        app.get_value(std::io::stdout(), &client)?
    }
    Ok(())
}
