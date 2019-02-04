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
    rc::Rc,
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

use speedruns::{types::*, Database};

pub type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::try_init_from_env(env_logger::Env::new().default_filter_or(format!(
        "{}=trace,speedruns=trace,hyper=debug",
        module_path!()
    )))?;

    let static_database = Box::leak(Box::new(unpack_bundled_database()));

    let mut server = speedruns::server::Server::new(static_database);
    server.run();

    Ok(())
}

fn load_data<T: DeserializeOwned>(
    reader: &mut &[u8],
    database: &mut Database,
    loader: impl Fn(&mut Database, T),
) -> Result<(), Box<dyn std::error::Error>> {
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
        loader(database, item?);
    }
    Ok(())
}

fn unpack_bundled_database() -> Database {
    let mut database = Database::new();

    trace!("Unpacking bundled database...");

    load_data(
        &mut include_bytes!("../../data/normalized/categories.bin.xz").as_ref(),
        &mut database,
        Database::insert_category,
    )
    .expect("category data corrupt");
    load_data(
        &mut include_bytes!("../../data/normalized/games.bin.xz").as_ref(),
        &mut database,
        Database::insert_game,
    )
    .expect("game data corrupt");
    load_data(
        &mut include_bytes!("../../data/normalized/levels.bin.xz").as_ref(),
        &mut database,
        Database::insert_level,
    )
    .expect("level data corrupt");
    load_data(
        &mut include_bytes!("../../data/normalized/runs.bin.xz").as_ref(),
        &mut database,
        Database::insert_run,
    )
    .expect("run data corrupt");
    load_data(
        &mut include_bytes!("../../data/normalized/users.bin.xz").as_ref(),
        &mut database,
        Database::insert_user,
    )
    .expect("user data corrupt");

    database.validate().expect("database state invalid");

    database
}
