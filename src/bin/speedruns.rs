#![feature(try_blocks)]
#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(
    missing_debug_implementations,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

use std::error::Error;

#[allow(unused)] use log::{debug, error, info, trace, warn};

mod normalize;
mod scrape;
mod serve;

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Tools for importing and serving some data from the speedrun.com API.
struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommand {
    Download(DownloadArgs),
    Import(ImportArgs),
    Serve(serve::Args),
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Fetches/updates a local mirror of speedrun.com API content. This
/// just stores the JSON representation of each item as-is, it doesn't
/// make any assumptions about their structure beyond the existence of
/// a string "id" value.
#[argh(subcommand, name = "download")]
struct DownloadArgs {}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// imports downloaded data (converting it to our internal representation, discarding weird
/// records). existing data is removed/replaced.
#[argh(subcommand, name = "import")]
struct ImportArgs {
    /// whether to skip most records and only import a small number, for faster testing.
    #[argh(switch)]
    skip_most: bool,
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Enable all debug logs by default.
    if std::env::var("RUST_LOG").unwrap_or_default().is_empty() {
        std::env::set_var("RUST_LOG", "debug");
    }

    pretty_env_logger::init();

    let args: Args = argh::from_env();

    match args.subcommand {
        Subcommand::Download(_args) => {
            scrape::main()?;
        }
        Subcommand::Import(_args) => {
            normalize::main()?;
        }
        Subcommand::Serve(args) => {
            serve::main(args).await?;
        }
    }

    Ok(())
}
