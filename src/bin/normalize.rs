//! Convert our API data into our simplified and normalized format.
#![warn(missing_debug_implementations, missing_docs)]
#![allow(unused_imports, missing_debug_implementations, missing_docs)]
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    error::Error,
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    num::NonZeroU64 as p64,
    ops::Deref,
    rc::Rc,
};

use chrono::{DateTime, NaiveDate, Utc};
use flate2::{read::GzDecoder, write::GzEncoder};
use getset::Getters;
use itertools::Itertools;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use tempfile::NamedTempFile;
use url::Url;
use validator::{Validate, ValidationError, ValidationErrors};
use validator_derive::Validate;
use xz2::write::XzEncoder;

use speedruns::{
    api_types as api, database::Database, normalize_api_types::Normalize,
    normalized_types::*, p64_from_base36, validators::*,
};

pub type BoxErr = Box<dyn std::error::Error>;

pub fn main() -> Result<(), BoxErr> {
    env_logger::try_init_from_env(
        env_logger::Env::new()
            .default_filter_or(format!("{}=trace,speedruns=trace", module_path!())),
    )?;

    let mut database = Database::new();

    info!("Loading API data...");
    load_api_type("data/api/games.jsonl.gz", &mut database, load_api_game)?;
    info!("Loaded {} API games.", database.games().len());
    load_api_type("data/api/users.jsonl.gz", &mut database, load_api_user)?;
    info!("Loaded {} API users.", database.users().len());
    load_api_type("data/api/runs.jsonl.gz", &mut database, load_api_run)?;
    info!("Loaded {} API runs.", database.runs().len());

    database.validate()?;

    info!("Dumping {} games...", database.games().len());
    dump_table("data/normalized/games", database.games())?;
    info!("Dumping {} users...", database.users().len());
    dump_table("data/normalized/users", database.users())?;
    info!("Dumping {} runs...", database.runs().len());
    dump_table("data/normalized/runs", database.runs())?;
    info!("Dumping {} categories...", database.categories().len());
    dump_table("data/normalized/categories", database.categories())?;
    info!("Dumping {} levels...", database.levels().len());
    dump_table("data/normalized/levels", database.levels())?;

    Ok(())
}

fn load_api_type<T: DeserializeOwned>(
    path: &str,
    database: &mut Database,
    loader: impl Fn(&mut Database, &T),
) -> Result<(), BoxErr> {
    let file = File::open(path)?;
    let buffer = BufReader::new(&file);
    let decompressor = GzDecoder::new(buffer);
    let deserializer = JsonDeserializer::from_reader(decompressor);
    let json_results = deserializer.into_iter::<JsonValue>();
    let items = json_results
        .map(Result::unwrap)
        .map(T::deserialize)
        .map(Result::unwrap);

    for item in items {
        loader(database, &item);
    }
    Ok(())
}

fn load_api_game(database: &mut Database, api_game: &api::Game) {
    let (game, categories, levels) = api_game.normalize().unwrap();
    database.insert_game(game);
    for category in categories {
        database.insert_category(category);
    }
    for level in levels {
        database.insert_level(level);
    }
}

fn load_api_user(database: &mut Database, api_user: &api::User) {
    let user = api_user.normalize().unwrap();
    database.insert_user(user);
}

fn load_api_run(database: &mut Database, api_run: &api::Run) {
    let optional_run = api_run.normalize().unwrap();
    if let Some(run) = optional_run {
        database.insert_run(run);
    }
}

fn dump_table<T: Serialize + Ord>(
    path: &str,
    table: &BTreeMap<p64, T>,
) -> Result<(), BoxErr> {
    let mut file = NamedTempFile::new_in("data")?;
    {
        let mut buffer = BufWriter::new(&mut file);
        for data in table.values().sorted() {
            serde_json::to_writer(&mut buffer, &data)?;
            buffer.write(b"\n")?;
        }
    }
    file.persist(format!("{}.jsonl", path))?;

    let mut file = NamedTempFile::new_in("data")?;
    {
        let buffer = BufWriter::new(&mut file);
        let mut compressor = XzEncoder::new(buffer, 9);
        for data in table.values().sorted() {
            bincode::serialize_into(&mut compressor, &data)?;
        }
        compressor.finish()?;
    }
    file.persist(format!("{}.bin.xz", path))?;

    Ok(())
}
