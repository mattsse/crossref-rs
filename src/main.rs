use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "crossref",
    about = "Access the crossref API from the command line."
)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
enum App {
    Query,
}

fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    pretty_env_logger::try_init()?;
    let _app = App::from_args();

    Ok(())
}
