use crossref::{Crossref, WorksQuery};
use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(
    name = "crossref",
    about = "Access the crossref API from the command line."
)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
enum App {
    #[structopt(name = "works", about = "Query crossref works")]
    Works {
        #[structopt(short = "d", long = "deep-page", help = "deep-page the request")]
        deep_page: bool,
        #[structopt(flatten)]
        opts: Opts,
        #[structopt(name = "for", subcommand)]
        for_: Option<Combined>,
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
        #[structopt(flatten)]
        opts: Opts,
    },
    #[structopt(name = "prefixes", about = "Query crossref prefixes")]
    Prefixes {
        #[structopt(flatten)]
        opts: Opts,
    },
    #[structopt(name = "types", about = "Query crossref types")]
    Types {
        #[structopt(flatten)]
        opts: Opts,
    },
}

impl App {
    pub fn opts(&self) -> &Opts {
        match self {
            App::Works { opts, .. }
            | App::Funders { opts, .. }
            | App::Members { opts, .. }
            | App::Journals { opts, .. }
            | App::Prefixes { opts, .. }
            | App::Types { opts, .. } => opts,
        }
    }
}

#[derive(Debug, StructOpt)]
enum Combined {
    #[structopt(name = "member", about = "Get Works of a specific Member")]
    Member { id: String },
    #[structopt(name = "funders", about = "Get Works of a specific Funder")]
    Funder { id: String },
    #[structopt(name = "journal", about = "Get Works of a specific Journal")]
    Journal { id: String },
    #[structopt(name = "prefix", about = "Get Works of a specific Prefix")]
    Prefix { id: String },
    #[structopt(name = "type", about = "Get Works of a specific Type")]
    Type { id: String },
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
    #[structopt(long = "polite", help = "The email to use for the polite pool")]
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

    #[structopt(short = "l", long = "limit", help = "limit the amount of results")]
    limit: Option<usize>,

    #[structopt(flatten)]
    client_opts: ClientOpts,
}

fn main() -> Result<(), failure::Error> {
    pretty_env_logger::try_init()?;
    let app = App::from_args();

    let client: Crossref = app.opts().client_opts.create_client()?;

    let work = client.works(WorksQuery::new("Machine Learning")).unwrap();

    println!("{:?}", work.items.len());

    let deep: Vec<_> = client
        .deep_page(WorksQuery::new("Machine Learning"))
        .into_work_iter()
        .take(45)
        .collect();

    println!("{:?}", deep.len());

    Ok(())
}
