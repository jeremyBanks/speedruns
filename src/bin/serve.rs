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
#[allow(unused)] use log::{debug, error, info, trace, warn};

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
    // Enable all debug logs by default.
    if std::env::var("RUST_LOG").unwrap_or_default().is_empty() {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    info!("Initializing server.");
    lazy_static::initialize(&DATABASE);

    info!("Initializing schema.");
    let schema = Arc::new(graphql::schema());

    let query = r#"
    fragment GameRun on Run {
        id
        # srcId
        # timeMs
        # category {
        #   id
        #   srcId
        #   __typename
        # }
        # level {
        #   id
        #   srcId
        #   __typename
        # }
        # date
        # players {
        #   name
        #   isGuest
        #   user {
        #     id
        #     srcId
        #     slug
        #     __typename
        #   }
        #   __typename
        # }
    }
    fragment GameLeaderboardRun on LeaderboardRun {
        rank
        isTied
        tiedRank
        run {
            id
            ...GameRun
        }
        __typename
    }
    query GetGamePage {
        game: game(slug: "wc2") {
            id
            srcId
            slug
            srcSlug
            name
            gameCategories {
            id
            srcId
            slug
            srcSlug
            name
            leaderboard {
                ...GameLeaderboardRun
            }
            progression {
                improvementMs
                run {
                ...GameRun
                }
                leaderboardRun {
                ...GameLeaderboardRun
                __typename
                }
                __typename
            }
            __typename
            }
            levels {
            id
            srcId
            slug
            srcSlug
            name
            leaderboard(categorySlug: "mission") {
                ...GameLeaderboardRun
                __typename
            }
            __typename
            }
            __typename
        }
    }
    "#;

    
    let (result, errors) = juniper::execute(
        query,
        None,
        &schema,
        &juniper::Variables::new(),
        &graphql::Context { database: DATABASE.clone() },
    )
    .unwrap();
}

fn unpack_bundled_tables() -> Tables {
    info!("Unpacking bundled database...");

    let runs = read_table("data/normalized/runs.jsonl").expect("run data corrupt");
    info!("{} runs.", runs.len());
    let users = read_table("data/normalized/users.jsonl").expect("user data corrupt");
    info!("{} users.", users.len());
    let games = read_table("data/normalized/games.jsonl").expect("game data corrupt");
    info!("{} games.", games.len());
    let categories =
        read_table("data/normalized/categories.jsonl").expect("category data corrupt");
    info!("{} categories.", categories.len());
    let levels = read_table("data/normalized/levels.jsonl").expect("level data corrupt");
    info!("{} levels.", levels.len());

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
            error!("Failed to load table: {:?}", err);
            Ok(vec![])
        }
    }
}
