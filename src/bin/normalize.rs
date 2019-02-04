//! A simplified and normalized data model, closer to what we might use in
//! a database.
#![warn(missing_debug_implementations, missing_docs)]
#![allow(unused_imports, missing_debug_implementations, missing_docs)]
use std::{
    collections::BTreeMap, convert::TryFrom, fmt::Debug, fs::File, io::BufReader,
    num::NonZeroU64 as p64, ops::Deref, rc::Rc,
};

use chrono::{DateTime, NaiveDate, Utc};
use flate2::read::GzDecoder;
use getset::Getters;
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use url::Url;
use validator::{Validate, ValidationError, ValidationErrors};
use validator_derive::Validate;

use speedruncom_data_tools::{api_types, normalized_types::*, validators::*};

pub type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    env_logger::try_init_from_env(
        env_logger::Env::new().default_filter_or(format!("{}=info", module_path!())),
    )?;

    let mut database = Database::new();
    database.load()?;
    database.validate()?;

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

    pub fn load(&mut self) -> Result<(), DynError> {
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
            for item in items {
                loader(database, &item);
            }
            Ok(())
        }

        info!("Loading...");
        load_api_type("data/api/games.jsonl.gz", self, Database::load_api_game)?;
        info!("Loaded {} games.", self.games().len());
        load_api_type("data/api/users.jsonl.gz", self, Database::load_api_user)?;
        info!("Loaded {} users.", self.users().len());
        load_api_type("data/api/runs.jsonl.gz", self, Database::load_api_run)?;
        info!("Loaded {} runs.", self.runs().len());

        info!("Done");
        Ok(())
    }

    fn id_from_str(id_str: &str) -> p64 {
        let mut id: u64 = 0;

        if id_str.bytes().len() > 8 {
            panic!("id too long");
        }

        for byte in id_str.bytes() {
            id = (id << 8) | u64::from(byte);
        }

        p64::new(id).unwrap()
    }

    pub fn load_api_game(&mut self, game: &api_types::Game) {
        let id = Self::id_from_str(&game.id());
        self.games.insert(id, Game { id });
    }

    pub fn load_api_user(&mut self, user: &api_types::User) {
        let id = Self::id_from_str(&user.id());
        let name = (user.names().international().clone())
            .or_else(|| user.names().twitch().clone())
            .or_else(|| user.names().japanese().clone())
            .expect("to have some name");
        let name = "".to_string();
        self.users.insert(id, User { id, name });
    }

    pub fn load_api_run(&mut self, run: &api_types::Run) {
        let id = Self::id_from_str(&run.id());
        // self.runs.insert(id, Run {
        //     id,
        // });
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

        validate_table(self.games())?;
        validate_table(self.users())?;
        validate_table(self.runs())?;
        validate_table(self.levels())?;
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
    /// Return all players of this run who are users.
    pub fn users(&self) -> Vec<User> {
        let mut users = Vec::<User>::new();
        for player in self.players() {
            if let RunPlayer::UserId(user_id) = player {
                users.push(
                    self.database()
                        .users()
                        .get(user_id)
                        .expect("user ID should be valid")
                        .clone(),
                );
            }
        }
        users
    }

    pub fn category(&self) -> Category {
        self.database
            .categories()
            .get(self.category_id())
            .expect("foreign key should be valid")
            .clone()
    }

    pub fn level(&self) -> Option<Level> {
        self.level_id().and_then(|level_id| {
            Some(
                self.database
                    .levels()
                    .get(&level_id)
                    .expect("foreign key should be valid")
                    .clone(),
            )
        })
    }

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
