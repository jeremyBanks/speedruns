#![feature(never_type)]
use bincode;
use flate2::read::GzDecoder;
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use std::{fs::File, io::BufReader};

use speedruncom_data_tools::api_types;

pub type DynError = Box<dyn std::error::Error>;

fn main() -> Result<!, DynError> {
    env_logger::try_init_from_env(
        env_logger::Env::default().filter_or("RUST_LOG", "structure=trace"),
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

    let file = File::open("data/run.jsonl.gz")?;
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

    std::process::exit(0)
}
