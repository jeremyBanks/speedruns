#![feature(proc_macro_hygiene)]
#![warn(missing_debug_implementations, missing_docs)]
#![allow(
    unused_imports,
    missing_debug_implementations,
    missing_docs,
    clippy::useless_attribute
)]
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    error::Error,
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader, BufWriter, Read},
    num::NonZeroU64 as Id64,
    ops::Deref,
    sync::Arc,
};

use futures::future;
use hyper::{
    header::HeaderValue,
    rt::{self, Future, Stream},
    service::{service_fn, service_fn_ok},
    Body, Method, Request, Response, Server, StatusCode,
};
use lazy_static::lazy_static;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use maud::{html, Markup, Render};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::Validate;
use xz2::read::XzDecoder;

use speedruns::{
    data::database::{Database, Tables},
    types::*,
};

pub type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::try_init_from_env(env_logger::Env::new().default_filter_or(format!(
        "{}=trace,speedruns=trace,hyper=debug",
        module_path!()
    )))?;

    let tables: &'static Tables = Box::leak(Box::new(unpack_bundled_tables()));
    let database: Arc<Database> = Database::new(tables).expect("database should be valid");

    let mut server = speedruns::server::Server::new(database);
    server.run();

    Ok(())
}

fn unpack_bundled_tables() -> Tables {
    trace!("Unpacking bundled database...");

    let runs =
        unpack_table(&mut include_bytes!("../../data/normalized/runs.bin.xz").as_ref())
            .expect("run data corrupt");

    let users =
        unpack_table(&mut include_bytes!("../../data/normalized/users.bin.xz").as_ref())
            .expect("user data corrupt");

    let games =
        unpack_table(&mut include_bytes!("../../data/normalized/games.bin.xz").as_ref())
            .expect("game data corrupt");

    let categories = unpack_table(
        &mut include_bytes!("../../data/normalized/categories.bin.xz").as_ref(),
    )
    .expect("category data corrupt");

    let levels =
        unpack_table(&mut include_bytes!("../../data/normalized/levels.bin.xz").as_ref())
            .expect("level data corrupt");

    Tables::new(runs, users, games, categories, levels)
}

fn unpack_table<T: DeserializeOwned>(
    reader: &mut &[u8],
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let mut items = vec![];
    let mut decompressor = XzDecoder::new(reader);
    loop {
        // We have left no way to detect the last item except for EOF.
        let item = bincode::deserialize_from::<_, T>(&mut decompressor);
        if let Err(ref error) = item {
            if let bincode::ErrorKind::Io(ref error) = **error {
                trace!("Assuming IO error is end of data EOF: {:?}", error);
                break
            }
        }
        items.push(item?);
    }
    Ok(items)
}
