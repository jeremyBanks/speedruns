#![feature(arbitrary_self_types, assoc_int_consts)]
#![allow(unused_imports)]
use async_std::{
    prelude::*,
    sync::{channel, Arc, RwLock},
    task::{sleep, spawn},
};
use itertools::Itertools;
use log::{debug, error, info, trace, warn};
use rand::prelude::*;
use std::{
    collections::{BTreeMap, HashSet},
    io::Write,
    rc::Rc,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Database {
    names: HashSet<String>,
}

#[derive(Debug)]
pub struct Index<'database> {
    /// a reference to name that's alphabetically first in the .names set
    alphabetically_first: &'database String,
}

#[macro_use] extern crate rental;
rental! {
    pub mod rent_index {
        #[rental(covariant, debug, covariant)]
        pub struct IndexedDatabase {
            database: Box<super::Database>,
            index: super::Index<'database>,
        }
    }
}
use rent_index::*;

impl IndexedDatabase {
    pub fn default() -> Self {
        IndexedDatabase::new(
            Box::new(Database {
                names: vec!["Chris", "Jeremy", "Katz", "Manish"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            }),
            |db| Index {
                alphabetically_first: db.names.iter().sorted().next().expect("empty? oops"),
            },
        )
    }
}

#[async_std::main]
async fn main() {
    init_logger();

    let idb = IndexedDatabase::default();

    debug!("{:#?}", idb.all().database);
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
