#![feature(try_blocks)]
#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(
    missing_debug_implementations,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![deny(unconditional_recursion)]

use std::{fs::File, io::BufReader, sync::Arc};

use juniper;
use lazy_static::lazy_static;

use serde::de::DeserializeOwned;
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};

use speedruns::data::{
    database::{Database, Tables},
    graphql,
};

lazy_static! {
    static ref DATABASE: Arc<Database> = {
        let tables: &'static Tables = Box::leak(Box::new(unpack_bundled_tables()));
        Database::new(tables).expect("database should be valid")
    };
}

fn main() {
    let schema = Arc::new(graphql::schema());

    let query = r#"
        fragment GameRun on Run {
            id
        }
        fragment GameLeaderboardRun on LeaderboardRun {
            run {
                ...GameRun
            }
        }
        query GetGamePage {
            game: game(slug: "wc2") {
                gameCategories {
                    leaderboard {
                        ...GameLeaderboardRun
                    }
                }
            }
        }
    "#;

    
    juniper::execute(
        query,
        None,
        &schema,
        &juniper::Variables::new(),
        &graphql::Context { database: DATABASE.clone() },
    ).unwrap();
}

fn unpack_bundled_tables() -> Tables {    let runs = read_table("data/normalized/runs.jsonl").expect("run data corrupt");
    let users = read_table("data/normalized/users.jsonl").expect("user data corrupt");
    let games = read_table("data/normalized/games.jsonl").expect("game data corrupt");
    let categories =
        read_table("data/normalized/categories.jsonl").expect("category data corrupt");
    let levels = read_table("data/normalized/levels.jsonl").expect("level data corrupt");
    Tables::new(runs, users, games, categories, levels)
}

pub fn read_table<T: DeserializeOwned>(
    path: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let result: Result<Vec<T>, Box<dyn std::error::Error>> = try {
        let file = File::open(path)?;
        let buffer = BufReader::new(&file);
        let deserializer = JsonDeserializer::from_reader(buffer);
        let json_results = deserializer.into_iter::<JsonValue>();
        json_results
            .map(Result::unwrap)
            .map(T::deserialize)
            .map(Result::unwrap)
            .collect()
    };
    match result {
        Ok(result) => Ok(result),
        Err(err) => {
            Ok(vec![])
        }
    }
}
