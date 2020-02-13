//! Convert our API data into our simplified and normalized format.bash.
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

use speedruns::{
    api::{self, normalize::Normalize},
    data::{
        database::{Database, IntegrityError, Tables},
        models::{AnyModel, AnyModelVec},
    },
};

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Imports downloaded data (converting it to our internal representation, discarding weird
/// records). existing data is removed/replaced. This is even less memory-efficient than
/// `download` because it also stores everything in memory, and but also memory leaks on top
/// of that!
#[argh(subcommand, name = "import")]
pub struct Args {
    /// import a subset of the API data into our fixtures, instead of importing the full
    /// data set into our database.
    #[argh(switch)]
    fixtures: bool,
}

pub fn main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut runs = Vec::new();
    let mut users = Vec::new();
    let mut games = Vec::new();
    let mut categories = Vec::new();
    let mut levels = Vec::new();

    if args.fixtures {
        info!("Generating fixture data, not importing into database.");
    }

    let fixture_game_slugs = ["wc1", "wc2", "wc2btdp", "bpr", "forza_horizon", "zoombinis"];
    let mut fixture_game_ids = HashSet::new();
    let mut fixture_run_ids = HashSet::new();
    let mut fixture_user_ids = HashSet::new();

    info!("Loading API games, with categories and levels...");
    for api_game in load_api_type::<api::Game>("data/api/games.jsonl.gz")? {
        if args.fixtures && !fixture_game_slugs.contains(&api_game.abbreviation().as_ref())
        {
            continue
        } else {
            fixture_game_ids.insert(api_game.id().clone());
        }

        let (game, mut game_categories, mut game_levels) = api_game
            .normalize()
            .expect("we should be able to handle all run game variations");

        games.push(game);
        categories.append(&mut game_categories);
        levels.append(&mut game_levels);
    }

    info!("Loading API runs...");
    for api_run in load_api_type::<api::Run>("data/api/runs.jsonl.gz")? {
        if args.fixtures && !fixture_game_ids.contains(api_run.game()) {
            continue
        } else {
            fixture_run_ids.insert(api_run.id().clone());
            for player in api_run.players() {
                if let api::RunPlayer::User { id, uri: _ } = player {
                    fixture_user_ids.insert(id.clone());
                }
            }
        }

        if let Some(run) = api_run
            .normalize()
            .expect("we should be able to handle all run data variations")
        {
            runs.push(run);
        }
    }

    info!("Loading API users...");
    for api_user in load_api_type::<api::User>("data/api/users.jsonl.gz")? {
        if args.fixtures && !fixture_user_ids.contains(api_user.id()) {
            continue
        }

        let user = api_user
            .normalize()
            .expect("we should be able to handle all user data variations");

        users.push(user);
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
                let mut dead_run_ids = HashSet::<u64>::new();
                let mut dead_game_ids = HashSet::<u64>::new();
                let mut dead_user_ids = HashSet::<u64>::new();
                let mut dead_level_ids = HashSet::<u64>::new();
                let mut dead_category_ids = HashSet::<u64>::new();

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
                    (dead_run_ids.len() * 100) / runs.len().max(1)
                );
                error!(
                    "{:6} ({:3}%) invalid users",
                    dead_user_ids.len(),
                    (dead_user_ids.len() * 100) / users.len().max(1)
                );
                error!(
                    "{:6} ({:3}%) invalid games",
                    dead_game_ids.len(),
                    (dead_game_ids.len() * 100) / games.len().max(1)
                );
                error!(
                    "{:6} ({:3}%) invalid categories",
                    dead_category_ids.len(),
                    (dead_category_ids.len() * 100) / categories.len().max(1)
                );
                error!(
                    "{:6} ({:3}%) invalid levels",
                    dead_level_ids.len(),
                    (dead_level_ids.len() * 100) / levels.len().max(1)
                );

                runs.retain(|x| !dead_run_ids.contains(x.id()));
                users.retain(|x| !dead_user_ids.contains(x.id()));
                games.retain(|x| !dead_game_ids.contains(x.id()));
                categories.retain(|x| !dead_category_ids.contains(x.id()));
                levels.retain(|x| !dead_level_ids.contains(x.id()));
            }
        }
    }

    let dir = if args.fixtures { "fixture" } else { "imported" };

    info!("Dumping {} games...", games.len());
    dump_table(&format!("data/{}/games", dir), games)?;
    info!("Dumping {} users...", users.len());
    dump_table(&format!("data/{}/users", dir), users)?;
    info!("Dumping {} runs...", runs.len());
    dump_table(&format!("data/{}/runs", dir), runs)?;
    info!("Dumping {} categories...", categories.len());
    dump_table(&format!("data/{}/categories", dir), categories)?;
    info!("Dumping {} levels...", levels.len());
    dump_table(&format!("data/{}/levels", dir), levels)?;

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

    Ok(())
}
