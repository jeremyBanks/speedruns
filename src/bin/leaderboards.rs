#![warn(missing_debug_implementations, missing_docs)]
#![allow(unused_imports, missing_debug_implementations, missing_docs)]
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    error::Error,
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader, BufWriter, Read},
    num::NonZeroU64 as p64,
    ops::Deref,
    rc::Rc,
};

#[allow(unused)] use log::{debug, error, info, trace, warn};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::Validate;
use xz2::read::XzDecoder;

use speedruncom_data_tools::{database::Database, normalized_types::*, DynError};

fn main() -> Result<(), DynError> {
    env_logger::try_init_from_env(env_logger::Env::new().default_filter_or(format!(
        "{}=trace,speedruncom_data_tools=trace",
        module_path!()
    )))?;

    let database = load_included_data()?;
    database.validate()?;

    let game = database
        .games()
        .values()
        .filter(|game| game.name() == "Celeste")
        .next()
        .expect("Celeste to be in database");

    let category = database
        .categories()
        .values()
        .filter(|category| category.game_id == game.id && category.name() == "Any%")
        .next()
        .expect("Celeste Any% to be in database");

    dbg!((game, category));

    Ok(())
}

fn load_data<T: DeserializeOwned>(
    reader: &mut &[u8],
    database: &mut Database,
    loader: impl Fn(&mut Database, T),
) -> Result<(), DynError> {
    let mut decompressor = XzDecoder::new(reader);
    loop {
        // We have left no way to detect the last item except for EOF.
        let item = bincode::deserialize_from::<_, T>(&mut decompressor);
        if let Err(ref error) = item {
            if let bincode::ErrorKind::Io(ref error) = **error {
                trace!("Assuming IO error is end of data EOF: {:?}", error);
                break
            }
        }
        loader(database, item?);
    }
    Ok(())
}

fn load_included_data() -> Result<Database, DynError> {
    let mut database = Database::new();

    load_data(
        &mut include_bytes!("../../data/normalized/categories.bin.xz").as_ref(),
        &mut database,
        Database::insert_category,
    )?;
    load_data(
        &mut include_bytes!("../../data/normalized/games.bin.xz").as_ref(),
        &mut database,
        Database::insert_game,
    )?;
    load_data(
        &mut include_bytes!("../../data/normalized/levels.bin.xz").as_ref(),
        &mut database,
        Database::insert_level,
    )?;
    load_data(
        &mut include_bytes!("../../data/normalized/runs.bin.xz").as_ref(),
        &mut database,
        Database::insert_run,
    )?;
    load_data(
        &mut include_bytes!("../../data/normalized/users.bin.xz").as_ref(),
        &mut database,
        Database::insert_user,
    )?;

    Ok(database)
}
