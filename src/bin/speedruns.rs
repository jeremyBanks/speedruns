#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(
    missing_debug_implementations,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

#[allow(unused)] use log::{debug, error, info, trace, warn};

use std::{collections::BTreeMap, rc::Rc, sync::Arc};

struct MyApp {
    database: Arc<Database>,
}

impl MyApp {
    // requirement: we need this to be able to keep running even if we update the
    // database, ideally in a consistent way.
    // so sleep for 5 seconds
    async fn handle_request(self) -> String {
        "hello world".to_string()
    }
}

struct Database {
    table_user:         BTreeMap<u64, User>,
    index_user_by_name: BTreeMap<String, u64>,
}

struct User {
    id:   u64,
    name: String,
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();

    if args.quiet {
        std::env::set_var("RUST_LOG", "error");
    } else if args.verbose {
        std::env::set_var("RUST_LOG", "debug,speedruns=trace");
    } else if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    let mut app = MyApp::new();

    //

    Ok(())
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Tools for importing and serving some data from the speedrun.com API.
pub struct Args {
    /// silence log output except for errors. overrides --verbose and RUST_LOG.
    #[argh(switch, short = 'q')]
    quiet: bool,

    /// enables maximum logging for our code and debug logging for dependencies. overrides
    /// RUST_LOG.
    #[argh(switch, short = 'v')]
    verbose: bool,
}
