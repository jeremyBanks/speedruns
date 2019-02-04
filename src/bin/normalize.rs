//! Convert our API data into our simplified and normalized format.
#![warn(missing_debug_implementations, missing_docs)]
#![allow(unused_imports, missing_debug_implementations, missing_docs)]
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    num::NonZeroU64 as p64,
    ops::Deref,
    rc::Rc,
};

use chrono::{DateTime, NaiveDate, Utc};
use flate2::{read::GzDecoder, write::GzEncoder};
use getset::Getters;
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use tempfile::NamedTempFile;
use url::Url;
use validator::{Validate, ValidationError, ValidationErrors};
use validator_derive::Validate;

use speedruncom_data_tools::{
    api_types, decode_base_36, normalized_types::*, validators::*,
};

pub type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    env_logger::try_init_from_env(
        env_logger::Env::new().default_filter_or(format!("{}=info", module_path!())),
    )?;

    let mut database = Database::new();
    database.load_api_data()?;
    database.validate()?;
    database.dump()?;

    Ok(())
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Getters)]
#[get = "pub"]
pub struct Database {
    runs: BTreeMap<p64, Run>,
    users: BTreeMap<p64, User>,
    games: BTreeMap<p64, Game>,
    categories: BTreeMap<p64, Category>,
    levels: BTreeMap<p64, Level>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_api_data(&mut self) -> Result<(), DynError> {
        fn load_api_type<T: DeserializeOwned>(
            path: &str,
            database: &mut Database,
            loader: impl Fn(&mut Database, &T),
        ) -> Result<(), DynError> {
            let file = File::open(path)?;
            let buffer = BufReader::new(&file);
            let decompressor = GzDecoder::new(buffer);
            let deserializer = JsonDeserializer::from_reader(decompressor);
            let json_results = deserializer.into_iter::<JsonValue>();
            let items = json_results
                .map(Result::unwrap)
                .map(T::deserialize)
                .map(Result::unwrap);

            // let items = items.take(1024);

            for item in items {
                loader(database, &item);
            }
            Ok(())
        }

        info!("Loading API data...");
        load_api_type("data/api/games.jsonl.gz", self, Database::load_api_game)?;
        info!("Loaded {} API games.", self.games().len());
        load_api_type("data/api/users.jsonl.gz", self, Database::load_api_user)?;
        info!("Loaded {} API users.", self.users().len());
        load_api_type("data/api/runs.jsonl.gz", self, Database::load_api_run)?;
        info!("Loaded {} API runs.", self.runs().len());

        info!("Done");
        Ok(())
    }

    pub fn dump(&mut self) -> Result<(), DynError> {
        fn dump_table<T: Serialize>(
            path: &str,
            table: &BTreeMap<p64, T>,
        ) -> Result<(), DynError> {
            let mut file = NamedTempFile::new_in("data")?;
            {
                let mut buffer = BufWriter::new(&mut file);
                // let mut compressor = GzEncoder::new(buffer, flate2::Compression::best());
                for data in table.values() {
                    serde_json::to_writer(&mut buffer, &data)?;
                    buffer.write(b"\n")?;
                }
                // compressor.finish()?;
            }
            file.persist(path)?;

            Ok(())
        }

        info!("Dumping {} games...", self.games().len());
        dump_table("data/normalized/games.jsonl", self.games())?;
        info!("Dumping {} users...", self.users().len());
        dump_table("data/normalized/users.jsonl", self.users())?;
        info!("Dumping {} runs...", self.runs().len());
        dump_table("data/normalized/runs.jsonl", self.runs())?;
        info!("Dumping {} categories...", self.categories().len());
        dump_table("data/normalized/categories.jsonl", self.categories())?;
        info!("Dumping {} levels...", self.levels().len());
        dump_table("data/normalized/levels.jsonl", self.levels())?;

        info!("Done");
        Ok(())
    }

    fn id_from_str(id_str: &str) -> p64 {
        p64::new(decode_base_36(id_str)).unwrap()
    }

    pub fn read_name(names: &api_types::Names) -> Option<String> {
        if let Some(name) = names.international() {
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
        if let Some(name) = names.twitch() {
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
        if let Some(name) = names.japanese() {
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
        None
    }

    pub fn load_api_game(&mut self, game: &api_types::Game) {
        let id = Self::id_from_str(&game.id());
        let slug = game.abbreviation().to_string();
        let name = Self::read_name(game.names()).unwrap_or(slug.clone());
        self.games.insert(id, Game { id, name, slug });
    }

    pub fn load_api_user(&mut self, user: &api_types::User) {
        let id = Self::id_from_str(&user.id());
        let name =
            Self::read_name(user.names()).unwrap_or_else(|| format!("Unknown User {}", id));
        self.users.insert(id, User { id, name });
    }

    pub fn load_api_run(&mut self, run: &api_types::Run) {
        let id = Self::id_from_str(run.id());
        let created = run.submitted().clone();
        let game_id = Self::id_from_str(run.game());
        self.runs.insert(
            id,
            Run {
                id,
                created,
                game_id,
            },
        );
    }
}

impl Validate for Database {
    fn validate(&self) -> Result<(), ValidationErrors> {
        fn validate_table<T: Validate + Debug>(
            table: &BTreeMap<p64, T>,
        ) -> Result<(), ValidationErrors> {
            for item in table.values() {
                let result = item.validate();
                if let Err(ref error) = result {
                    error!("{} in {:?}", &error, &item);
                }
                result?;
            }
            Ok(())
        }

        // TODO:
        // foreign keys
        // unique constraints
        // indexed by id

        info!("Validating normalized games...");
        validate_table(self.games())?;
        info!("Validating normalized users...");
        validate_table(self.users())?;
        info!("Validating normalized runs...");
        validate_table(self.runs())?;
        info!("Validating normalized levels...");
        validate_table(self.levels())?;
        info!("Validating normalized categories...");
        validate_table(self.categories())?;

        Ok(())
    }
}

#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct DbRun {
    database: Rc<Database>,
    run: Run,
}

impl DbRun {
    // /// Return all players of this run who are users.
    // pub fn users(&self) -> Vec<User> {
    //     let mut users = Vec::<User>::new();
    //     for player in self.players() {
    //         if let RunPlayer::UserId(user_id) = player {
    //             users.push(
    //                 self.database()
    //                     .users()
    //                     .get(user_id)
    //                     .expect("user ID should be valid")
    //                     .clone(),
    //             );
    //         }
    //     }
    //     users
    // }

    // pub fn category(&self) -> Category {
    //     self.database
    //         .categories()
    //         .get(self.category_id())
    //         .expect("foreign key should be valid")
    //         .clone()
    // }

    // pub fn level(&self) -> Option<Level> {
    //     self.level_id().and_then(|level_id| {
    //         Some(
    //             self.database
    //                 .levels()
    //                 .get(&level_id)
    //                 .expect("foreign key should be valid")
    //                 .clone(),
    //         )
    //     })
    // }

    pub fn game(&self) -> Game {
        self.database
            .games()
            .get(self.game_id())
            .expect("foreign key should be valid")
            .clone()
    }
}

impl Deref for DbRun {
    type Target = Run;
    fn deref(&self) -> &Run {
        &self.run
    }
}
