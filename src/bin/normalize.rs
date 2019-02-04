//! A simplified and normalized data model, closer to what we might use in
//! a database.
#![warn(missing_debug_implementations, missing_docs)]
#![allow(unused_imports, missing_debug_implementations, missing_docs)]
use std::{fs::File, io::BufReader, ops::Deref};

use chrono::{DateTime, NaiveDate, Utc};
use getset::Getters;
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, num::NonZeroU64 as p64, rc::Rc};
use url::Url;
use validator::{Validate, ValidationError, ValidationErrors};
use validator_derive::Validate;

use speedruncom_data_tools::api_types;

pub type DynError = Box<dyn std::error::Error>;

trait IsDefault: Default + PartialEq {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl IsDefault for &str {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsDefault for Vec<T>
where
    T: PartialEq,
{
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

fn nondefault(value: impl IsDefault) -> Result<(), ValidationError> {
    if value.is_default() {
        Err(ValidationError::new("value is uninitialized"))
    } else {
        Ok(())
    }
}

fn urls(value: &[String]) -> Result<(), ValidationError> {
    for item in value {
        if !validator::validate_url(item) {
            return Err(ValidationError::new("invalid URL"));
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
#[get = "pub"]
pub struct Database {
    runs: BTreeMap<p64, Run>,
    users: BTreeMap<p64, User>,
    games: BTreeMap<p64, Game>,
    categories: BTreeMap<p64, Category>,
    levels: BTreeMap<p64, Level>,
}

impl Validate for Database {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

#[derive(
    Debug, Serialize, Deserialize, PartialEq, Hash, Clone, Getters, Validate,
)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct Run {
    id: p64,
    created: Option<DateTime<Utc>>,
    level_id: Option<p64>,
    game_id: p64,
    category_id: p64,
    players: Vec<RunPlayer>,
    date: Option<NaiveDate>,
    time_ms: u64,
    comment: String,
    #[validate(custom = "urls")]
    video_urls: Vec<String>,
    #[validate(custom = "urls")]
    split_urls: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub enum RunPlayer {
    UserId(p64),
    GuestName(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub id: p64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Category {
    pub id: p64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Level {
    pub id: p64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Game {
    pub id: p64,
}

fn main() -> Result<(), DynError> {
    env_logger::try_init_from_env(
        env_logger::Env::new()
            .default_filter_or(format!("{}=trace", module_path!())),
    )?;

    Ok(())
}
