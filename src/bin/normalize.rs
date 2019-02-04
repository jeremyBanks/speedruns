//! Convert our API data into our simplified and normalized format.
#![warn(missing_debug_implementations, missing_docs)]
#![allow(clippy::useless_attribute)]
use std::{
    collections::BTreeMap,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    num::NonZeroU64 as Id64,
};

use flate2::read::GzDecoder;
use itertools::Itertools;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use tempfile::NamedTempFile;
use xz2::write::XzEncoder;

use speedruns::{
    api::{self, normalize::Normalize},
    data::base::{Database, Tables},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::try_init_from_env(
        env_logger::Env::new()
            .default_filter_or(format!("{}=trace,speedruns=trace", module_path!())),
    )?;

    let mut runs = Vec::new();
    let mut users = Vec::new();
    let mut games = Vec::new();
    let mut categories = Vec::new();
    let mut levels = Vec::new();

    info!("Loading API runs...");
    for api_run in load_api_type::<api::Run>("data/api/runs.jsonl.gz")? {
        if let Some(run) = api_run.normalize().unwrap() {
            runs.push(run);
        }
    }

    info!("Loading API users...");
    for api_user in load_api_type::<api::User>("data/api/users.jsonl.gz")? {
        let user = api_user.normalize().unwrap();
        users.push(user);
    }

    info!("Loading API games, with categories and levels...");
    for api_game in load_api_type::<api::Game>("data/api/games.jsonl.gz")? {
        let (game, mut game_categories, mut game_levels) = api_game.normalize().unwrap();
        games.push(game);
        categories.append(&mut game_categories);
        levels.append(&mut game_levels);
    }

    let tables = Box::leak(Box::new(Tables::new(
        runs, users, games, categories, levels,
    )));

    info!("Validating API data...");

    while let Err(error) = Database::new(tables) {
        error!("Database validation failed: {}", error);
    }

    info!("Dumping {} games...", tables.games().len());
    dump_table("data/normalized/games", tables.games())?;
    info!("Dumping {} users...", tables.users().len());
    dump_table("data/normalized/users", tables.users())?;
    info!("Dumping {} runs...", tables.runs().len());
    dump_table("data/normalized/runs", tables.runs())?;
    info!("Dumping {} categories...", tables.categories().len());
    dump_table("data/normalized/categories", tables.categories())?;
    info!("Dumping {} levels...", tables.levels().len());
    dump_table("data/normalized/levels", tables.levels())?;

    Ok(())
}

fn load_api_type<ApiType: DeserializeOwned>(
    path: &str,
) -> Result<Vec<ApiType>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(&file);
    let decompressor = GzDecoder::new(buffer);
    let deserializer = JsonDeserializer::from_reader(decompressor);
    let json_results = deserializer.into_iter::<JsonValue>();
    Ok(json_results
        .map(Result::unwrap)
        .map(ApiType::deserialize)
        .map(Result::unwrap)
        .collect())
}

fn dump_table<T: Serialize + Ord>(
    path: &str,
    table: &BTreeMap<Id64, T>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new_in("data")?;
    {
        let mut buffer = BufWriter::new(&mut file);
        for data in table.values().sorted() {
            serde_json::to_writer(&mut buffer, &data)?;
            buffer.write_all(b"\n")?;
        }
    }
    file.persist(format!("{}.jsonl", path))?;

    let mut file = NamedTempFile::new_in("data")?;
    {
        let buffer = BufWriter::new(&mut file);
        let mut compressor = XzEncoder::new(buffer, 6);
        for data in table.values().sorted() {
            bincode::serialize_into(&mut compressor, &data)?;
        }
        compressor.finish()?;
    }
    file.persist(format!("{}.bin.xz", path))?;

    Ok(())
}
