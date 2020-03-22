#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(
    missing_debug_implementations,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

use std::error::Error;

use speedruns_api::cli::{download, import};
use speedruns_juniper::cli as juniper_cli;

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Tools for importing and serving some data from the speedrun.com API.
pub struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,

    /// silence log output except for errors. overrides --verbose and RUST_LOG.
    #[argh(switch, short = 'q')]
    quiet: bool,

    /// enables maximum logging for our code and debug logging for dependencies. overrides
    /// RUST_LOG.
    #[argh(switch, short = 'v')]
    verbose: bool,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Subcommand {
    Download(DownloadArgs),
    Import(import::Args),
    Serve(juniper_cli::Args),
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Fetches/updates a local mirror of speedrun.com API content. This just stores the JSON
/// representation of each item as-is, it doesn't make any assumptions about their structure
/// beyond the existence of  a string "id" value. This stores everything in-memory, it's not
/// memory-efficient.
#[argh(subcommand, name = "download")]
pub struct DownloadArgs {}

pub async fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = argh::from_env();

    if args.quiet {
        std::env::set_var("RUST_LOG", "error");
    } else if args.verbose {
        std::env::set_var("RUST_LOG", "debug,speedruns=trace");
    } else if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    match args.subcommand {
        Subcommand::Download(_args) => {
            download::main()?;
        }
        Subcommand::Import(args) => {
            import::main(args)?;
        }
        Subcommand::Serve(args) => {
            juniper_cli::main(args).await?;
        }
    }

    Ok(())
}
