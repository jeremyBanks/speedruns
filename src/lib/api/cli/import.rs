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
    sync::Arc,
};

use flate2::read::GzDecoder;
use itertools::Itertools;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use tempfile::NamedTempFile;

use crate::normalize::Normalize;
use speedruns_database::{Database, Tables};

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

pub fn main(args: Args) -> anyhow::Result<()> {
    let mut runs = Vec::new();
    let mut users = Vec::new();
    let mut games = Vec::new();
    let mut categories = Vec::new();
    let mut levels = Vec::new();

    if args.fixtures {
        log::info!("Generating fixture data, not importing into database.");
    }

    let fixture_game_slugs = ["wc1", "wc2", "wc2btdp", "bpr", "forza_horizon", "zoombinis"];
    let mut fixture_game_ids = HashSet::new();
    let mut fixture_run_ids = HashSet::new();
    let mut fixture_user_ids = HashSet::new();

    log::info!("Loading API games, with categories and levels...");
    for api_game in load_api_type::<crate::types::Game>("data/api/games.jsonl.gz")? {
        if args.fixtures && !fixture_game_slugs.contains(&api_game.abbreviation().as_ref())
        {
            continue;
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

    log::info!("Loading API runs...");
    for api_run in load_api_type::<crate::types::Run>("data/api/runs.jsonl.gz")? {
        if args.fixtures && !fixture_game_ids.contains(api_run.game()) {
            continue;
        } else {
            fixture_run_ids.insert(api_run.id().clone());
            for player in api_run.players() {
                if let crate::types::RunPlayer::User { id, .. } = player {
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

    log::info!("Loading API users...");
    for api_user in load_api_type::<crate::types::User>("data/api/users.jsonl.gz")? {
        if args.fixtures && !fixture_user_ids.contains(api_user.id()) {
            continue;
        }

        let user = api_user
            .normalize()
            .expect("we should be able to handle all user data variations");

        users.push(user);
    }

    log::info!("Validating and cleaning API data...");

    let database = Database::new(Arc::new(Tables::new(
        games, categories, levels, runs, users,
    )));

    let games: Vec<_> = database.games().values().collect();
    let categories: Vec<_> = database.categories().values().collect();
    let levels: Vec<_> = database.levels().values().collect();
    let runs: Vec<_> = database.runs().values().collect();
    let users: Vec<_> = database.users().values().collect();

    let dir = if args.fixtures { "fixture" } else { "imported" };

    log::info!("Dumping {} games...", games.len());
    dump_table(&format!("data/{}/games", dir), games)?;
    log::info!("Dumping {} users...", users.len());
    dump_table(&format!("data/{}/users", dir), users)?;
    log::info!("Dumping {} runs...", runs.len());
    dump_table(&format!("data/{}/runs", dir), runs)?;
    log::info!("Dumping {} categories...", categories.len());
    dump_table(&format!("data/{}/categories", dir), categories)?;
    log::info!("Dumping {} levels...", levels.len());
    dump_table(&format!("data/{}/levels", dir), levels)?;

    Ok(())
}

fn load_api_type<ApiType: DeserializeOwned>(path: &str) -> anyhow::Result<Vec<ApiType>> {
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

fn dump_table<T: Serialize + Ord>(path: &str, table: Vec<T>) -> anyhow::Result<()> {
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
