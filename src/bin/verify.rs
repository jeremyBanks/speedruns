//! Verify that the data we have in jsonl.gz files matches the expected
//! structure of our API types.
use env_logger;
use flate2::read::GzDecoder;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use serde::de::DeserializeOwned;
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use std::{fs::File, io::BufReader};

use speedruncom_data_tools::api_types;

pub type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    env_logger::try_init_from_env(
        env_logger::Env::new().default_filter_or(format!("{}=trace", module_path!())),
    )?;

    fn verify<T: DeserializeOwned>(path: &str, label: &str) -> Result<(), DynError> {
        let file = File::open(path)?;
        let buffer = BufReader::new(&file);
        let decompressor = GzDecoder::new(buffer);
        let deserializer = JsonDeserializer::from_reader(decompressor);
        let iterator = deserializer.into_iter::<JsonValue>();
        let mut count = 0;
        for data in iterator {
            count += 1;
            let data = data?;
            let game_result = T::deserialize(&data);
            if let Err(ref err) = game_result {
                panic!(
                    "{:#?} deserializing {}",
                    err,
                    serde_json::to_string(&data).unwrap()
                );
            }
        }
        info!("Deserialized {} {}.", count, label);
        Ok(())
    }

    verify::<api_types::Game>("data/api/games.jsonl.gz", "games")?;
    verify::<api_types::User>("data/api/users.jsonl.gz", "users")?;
    verify::<api_types::Run>("data/api/runs.jsonl.gz", "runs")?;

    Ok(())
}
