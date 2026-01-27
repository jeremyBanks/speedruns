#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(
    missing_debug_implementations,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

use std::error::Error;

use clap::{Parser, Subcommand as ClapSubcommand};
use log::warn;

use speedruns_api::cli::{download, import};
use speedruns_juniper::cli as juniper_cli;

#[derive(Parser, Debug)]
#[command(about = "Tools for importing and serving some data from the speedrun.com API.")]
pub struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,

    /// Silence log output except for errors. Overrides --verbose and RUST_LOG.
    #[arg(short = 'q', long)]
    quiet: bool,

    /// Enables maximum logging for our code and debug logging for dependencies. Overrides
    /// RUST_LOG.
    #[arg(short = 'v', long)]
    verbose: bool,
}

#[derive(ClapSubcommand, Debug)]
pub enum Subcommand {
    /// Fetches/updates a local mirror of speedrun.com API content.
    Download(DownloadArgs),
    /// Imports downloaded data (converting it to our internal representation).
    Import(import::Args),
    /// Serves imported data from a GraphQL server.
    Serve(juniper_cli::Args),
}

#[derive(Parser, Debug)]
/// Fetches/updates a local mirror of speedrun.com API content. This just stores the JSON
/// representation of each item as-is, it doesn't make any assumptions about their structure
/// beyond the existence of a string "id" value. This stores everything in-memory, it's not
/// memory-efficient.
pub struct DownloadArgs {
    /// Limit number of games to fetch runs for. -1 means unlimited (default).
    #[arg(long, default_value = "-1")]
    limit: i32,

    /// Backup existing data files with YYYYMMDDHHMM- prefix before downloading.
    #[arg(long)]
    backup: bool,
}

pub async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.quiet {
        std::env::set_var("RUST_LOG", "error");
    } else if args.verbose {
        std::env::set_var("RUST_LOG", "debug,speedruns=trace");
    } else if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    match args.subcommand {
        Subcommand::Download(args) => {
            download::main(args.limit, args.backup).await?;
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
