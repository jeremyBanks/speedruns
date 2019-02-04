//! Convert our API data into our simplified and normalized format.
#![warn(missing_debug_implementations, missing_docs)]
#![allow(clippy::useless_attribute)]
use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
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
    data::{
        base::{Database, IntegrityError, Tables},
        types::Id64,
    },
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

    info!("Validating API data...");

    loop {
        // memory leak, so hopefully not many iterations!
        match Database::new(Box::leak(Box::new(Tables::new(
            runs.clone(),
            users.clone(),
            games.clone(),
            categories.clone(),
            levels.clone(),
        )))) {
            Ok(_) => break,
            Err(errors) => {
                error!("Database validation failed: {}", errors);
                let mut invalid_run_ids = HashSet::<Id64>::new();
                let mut invalid_game_ids = HashSet::<Id64>::new();
                let mut invalid_user_ids = HashSet::<Id64>::new();
                let mut invalid_level_ids = HashSet::<Id64>::new();
                let mut invalid_category_ids = HashSet::<Id64>::new();

                for error in errors.errors {
                    match error {
                        IntegrityError::ForeignKeyMissing {
                            source_type,
                            source_id,
                            ..
                        } => {
                            match source_type {
                                "run" => &mut invalid_run_ids,
                                "user" => &mut invalid_user_ids,
                                "game" => &mut invalid_game_ids,
                                "level" => &mut invalid_level_ids,
                                "category" => &mut invalid_category_ids,
                                _ =>
                                    unreachable!("invalid source_type in validation error"),
                            }
                            .insert(source_id);
                        }
                        IntegrityError::IndexingError => {
                            // there should be a ForeignKeyMissing to cover this case
                        }
                        IntegrityError::CheckFailed { .. } => {
                            panic!("in-row validation error? shouldn't happen! normalization bug!");
                        }
                    }
                }

                error!(
                    "{:6} ({:3}%) invalid runs",
                    invalid_run_ids.len(),
                    (invalid_run_ids.len() * 100) / runs.len()
                );
                error!(
                    "{:6} ({:3}%) invalid users",
                    invalid_user_ids.len(),
                    (invalid_user_ids.len() * 100) / users.len()
                );
                error!(
                    "{:6} ({:3}%) invalid games",
                    invalid_game_ids.len(),
                    (invalid_game_ids.len() * 100) / games.len()
                );
                error!(
                    "{:6} ({:3}%) invalid categories",
                    invalid_category_ids.len(),
                    (invalid_category_ids.len() * 100) / categories.len()
                );
                error!(
                    "{:6} ({:3}%) invalid levels",
                    invalid_level_ids.len(),
                    (invalid_level_ids.len() * 100) / levels.len()
                );

                error!("dropping invalid items and building again.");

                runs.retain(|x| !invalid_run_ids.contains(x.id()));
                users.retain(|x| !invalid_user_ids.contains(x.id()));
                games.retain(|x| !invalid_game_ids.contains(x.id()));
                categories.retain(|x| !invalid_category_ids.contains(x.id()));
                levels.retain(|x| !invalid_level_ids.contains(x.id()));
            }
        }
    }

    info!("Dumping {} games...", games.len());
    dump_table("data/normalized/games", games)?;
    info!("Dumping {} users...", users.len());
    dump_table("data/normalized/users", users)?;
    info!("Dumping {} runs...", runs.len());
    dump_table("data/normalized/runs", runs)?;
    info!("Dumping {} categories...", categories.len());
    dump_table("data/normalized/categories", categories)?;
    info!("Dumping {} levels...", levels.len());
    dump_table("data/normalized/levels", levels)?;

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
    table: Vec<T>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new_in("data")?;
    {
        let mut buffer = BufWriter::new(&mut file);
        for data in table.iter().sorted() {
            serde_json::to_writer(&mut buffer, &data)?;
            buffer.write_all(b"\n")?;
        }
    }
    file.persist(format!("{}.jsonl", path))?;

    let mut file = NamedTempFile::new_in("data")?;
    {
        let buffer = BufWriter::new(&mut file);
        let mut compressor = XzEncoder::new(buffer, 6);
        for data in table.iter().sorted() {
            bincode::serialize_into(&mut compressor, &data)?;
        }
        compressor.finish()?;
    }
    file.persist(format!("{}.bin.xz", path))?;

    Ok(())
}
