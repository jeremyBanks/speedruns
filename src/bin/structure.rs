#![feature(never_type)]
#![allow(dead_code)]
use bincode;
use chrono::{DateTime, NaiveDate, Utc};
use flate2::{read::GzDecoder, write::GzEncoder};
use getset::{Getters, MutGetters, Setters};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::{
    map::Map as JsonMap, Deserializer as JsonDeserializer, Value as JsonValue,
};
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
};
use tempfile::NamedTempFile;

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

    // let file = File::create("data/games.bin.gz")?;
    // let buffer = BufWriter::new(&file);
    // let mut compressor = GzEncoder::new(buffer, Default::default());

    for (i, data) in iterator.enumerate() {
        let data = data?;
        match api_types::Game::deserialize(&data) {
            Ok(game) => {
                // bincode::serialize_into(&mut compressor, &game)?;
            }
            Err(err) => {
                error!(
                    "{:#?} deserializing {}",
                    err,
                    serde_json::to_string(&data).unwrap()
                );
                break;
            }
        }
    }

    info!("Done!");

    std::process::exit(0)
}
