#![feature(arbitrary_self_types, assoc_int_consts)]
use itertools::Itertools;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use owning_ref::ArcRef;
use std::{collections::HashSet, io::Write, sync::Arc, time::Instant};

#[derive(Debug)]
pub struct Database {
    names: HashSet<String>,
}

#[derive(Debug)]
pub struct Index<'database> {
    /// a reference to name that's alphabetically first in the .names set
    alphabetically_first: &'database String,
}

#[async_std::main]
async fn main() {
    init_logger();

    let idb = ArcRef::new(Arc::new(Database {
        names: vec!["Chris", "Jeremy", "Katz", "Manish"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }));

    let alphabetically_first = idb
        .clone()
        .map(|db| db.names.iter().sorted().next().expect("empty? oops"));

    let alphabetically_last = idb
        .clone()
        .map(|db| db.names.iter().sorted().rev().next().expect("empty? oops"));

    debug!("{:#?}", alphabetically_first);
    debug!("{:#?}", alphabetically_last);
}

fn init_logger() {
    // Configure a logger displaying elapsed time since now.
    let start = Instant::now();
    env_logger::Builder::new()
        .parse_filters("example=debug")
        .format(move |buffer, record| {
            let elapsed = (Instant::now() - start).as_secs_f64();
            writeln!(buffer, "[t={:>6.1}s] {}", elapsed, record.args())
        })
        .init();
}
