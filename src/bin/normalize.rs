//! Convert our API data into our simplified and normalized format.
#![allow(
    clippy::useless_attribute,
    clippy::cognitive_complexity,
    clippy::clone_on_copy
)]
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
        database::{Database, IntegrityError, Tables},
        models::{AnyModel, AnyModelVec},
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

    info!("Validating and cleaning API data...");

    loop {
        // memory leak, so hopefully not many iterations!
        match Database::new(Box::leak(Box::new(Tables::new(
            runs.clone(),
            users.clone(),
            games.clone(),
            categories.clone(),
            levels.clone(),
        )))) {
            Ok(_) => {
                info!("Database validation successful.");
                break
            }
            Err(errors) => {
                error!("Database validation failed: {}", errors);
                let mut dead_run_ids = HashSet::<Id64>::new();
                let mut dead_game_ids = HashSet::<Id64>::new();
                let mut dead_user_ids = HashSet::<Id64>::new();
                let mut dead_level_ids = HashSet::<Id64>::new();
                let mut dead_category_ids = HashSet::<Id64>::new();

                for error in errors.errors {
                    match error {
                        IntegrityError::IndexingError => {
                            error!("indexing failed");
                        }
                        IntegrityError::ForeignKeyMissing { source, .. } => {
                            use AnyModel::*;
                            match source {
                                Run(run) => dead_run_ids.insert(*run.id()),
                                User(user) => dead_user_ids.insert(*user.id()),
                                Game(game) => dead_game_ids.insert(*game.id()),
                                Level(level) => dead_level_ids.insert(*level.id()),
                                Category(category) =>
                                    dead_category_ids.insert(*category.id()),
                            };
                        }
                        IntegrityError::CheckFailed { .. } => {
                            panic!("value incorrectly normalized and fails validation?!");
                        }
                        IntegrityError::NonUniqueSlug { sources, .. } => {
                            use AnyModelVec::*;
                            match sources {
                                Runs(_) => unreachable!("runs don't have slugs?!"),
                                Games(games) => {
                                    let dead_dupes = games
                                        .iter()
                                        .sorted_by_key(|game| {
                                            (
                                                game.created(),
                                                game.slug().len(),
                                                game.name().len(),
                                                game.name(),
                                                game.id(),
                                            )
                                                .clone()
                                        })
                                        .skip(1);
                                    for dupe in dead_dupes {
                                        dead_game_ids.insert(*dupe.id());
                                    }
                                }
                                Users(users) => {
                                    let dead_dupes = users
                                        .iter()
                                        .sorted_by_key(|user| {
                                            (
                                                user.created(),
                                                user.name().len(),
                                                user.name(),
                                                user.id(),
                                            )
                                                .clone()
                                        })
                                        .skip(1);
                                    for dupe in dead_dupes {
                                        dead_user_ids.insert(*dupe.id());
                                    }
                                }
                                Categories(categories) => {
                                    let dead_dupes = categories
                                        .iter()
                                        .sorted_by_key(|category| {
                                            (
                                                category.name().len(),
                                                category.name(),
                                                category.id(),
                                            )
                                                .clone()
                                        })
                                        .skip(1);
                                    for dupe in dead_dupes {
                                        dead_category_ids.insert(*dupe.id());
                                    }
                                }
                                Levels(levels) => {
                                    let dead_dupes = levels
                                        .iter()
                                        .sorted_by_key(|level| {
                                            (level.name().len(), level.name(), level.id())
                                                .clone()
                                        })
                                        .skip(1);
                                    for dupe in dead_dupes {
                                        dead_level_ids.insert(*dupe.id());
                                    }
                                }
                            };
                        }
                        IntegrityError::MissingPrimaryTiming(run) => {
                            dead_run_ids.insert(*run.id());
                        }
                    }
                }

                error!(
                    "{:6} ({:3}%) invalid runs",
                    dead_run_ids.len(),
                    (dead_run_ids.len() * 100) / runs.len()
                );
                error!(
                    "{:6} ({:3}%) invalid users",
                    dead_user_ids.len(),
                    (dead_user_ids.len() * 100) / users.len()
                );
                error!(
                    "{:6} ({:3}%) invalid games",
                    dead_game_ids.len(),
                    (dead_game_ids.len() * 100) / games.len()
                );
                error!(
                    "{:6} ({:3}%) invalid categories",
                    dead_category_ids.len(),
                    (dead_category_ids.len() * 100) / categories.len()
                );
                error!(
                    "{:6} ({:3}%) invalid levels",
                    dead_level_ids.len(),
                    (dead_level_ids.len() * 100) / levels.len()
                );

                runs.retain(|x| !dead_run_ids.contains(x.id()));
                users.retain(|x| !dead_user_ids.contains(x.id()));
                games.retain(|x| !dead_game_ids.contains(x.id()));
                categories.retain(|x| !dead_category_ids.contains(x.id()));
                levels.retain(|x| !dead_level_ids.contains(x.id()));
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
