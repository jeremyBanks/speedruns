//! Verify that the data we have in jsonl.gz files matches the structure of our
//! API types.
#![feature(never_type)]
use env_logger;
use flate2::read::GzDecoder;
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use speedruncom_data_tools::api_types;
use std::{fs::File, io::BufReader};

pub type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    env_logger::try_init_from_env(
        env_logger::Env::new()
            .default_filter_or(format!("{}=trace", module_path!())),
    )?;

    let file = File::open("data/games.jsonl.gz")?;
    let buffer = BufReader::new(&file);
    let decompressor = GzDecoder::new(buffer);
    let deserializer = JsonDeserializer::from_reader(decompressor);
    let iterator = deserializer.into_iter::<JsonValue>();
    for data in iterator {
        let data = data?;
        let game_result = api_types::Game::deserialize(&data);
        if let Err(err) = game_result {
            error!(
                "{:#?} deserializing {}",
                err,
                serde_json::to_string(&data).unwrap()
            );
            return Err(err.into());
        }
    }
    info!("Deserialized all games.");

    let file = File::open("data/users.jsonl.gz")?;
    let buffer = BufReader::new(&file);
    let decompressor = GzDecoder::new(buffer);
    let deserializer = JsonDeserializer::from_reader(decompressor);
    let iterator = deserializer.into_iter::<JsonValue>();
    for data in iterator {
        let data = data?;
        let user_result = api_types::User::deserialize(&data);
        if let Err(err) = user_result {
            error!(
                "{:#?} deserializing {}",
                err,
                serde_json::to_string(&data).unwrap()
            );
            return Err(err.into());
        }
    }
    info!("Deserialized all users.");

    let file = File::open("data/runs.jsonl.gz")?;
    let buffer = BufReader::new(&file);
    let decompressor = GzDecoder::new(buffer);
    let deserializer = JsonDeserializer::from_reader(decompressor);
    let iterator = deserializer.into_iter::<JsonValue>();
    for data in iterator {
        let data = data?;
        let run_result = api_types::Run::deserialize(&data);
        if let Err(err) = run_result {
            error!(
                "{:#?} deserializing {}",
                err,
                serde_json::to_string(&data).unwrap()
            );
            return Err(err.into());
        }
    }
    info!("Deserialized all runs.");

    Ok(())
}
