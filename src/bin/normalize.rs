use chrono::{DateTime, NaiveDate, Utc};
use getset::Getters;
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, rc::Rc};
use url::Url;

pub type DynError = Box<dyn std::error::Error>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct Database {
    pub runs: BTreeMap<u64, Rc<Run>>,
    pub users: BTreeMap<u64, Rc<User>>,
    pub games: BTreeMap<u64, Rc<Game>>,
    pub categories: BTreeMap<u64, Rc<Category>>,
    pub levels: BTreeMap<u64, Rc<Level>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Run {
    pub id: u64,
    pub date: Option<NaiveDate>,
    pub created: Option<DateTime<Utc>>,
    pub game_id: u64,
    pub category_id: u64,
    pub level_id: Option<u64>,
    pub time_ms: u64,
    pub players: Vec<RunPlayer>,
    pub comment: String,
    pub video_urls: Vec<Url>,
    pub split_urls: Vec<Url>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub enum RunPlayer {
    UserId(u64),
    GuestName(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Category {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Level {
    pub id: u64,
}

fn main() -> Result<(), DynError> {
    env_logger::try_init_from_env(
        env_logger::Env::new()
            .default_filter_or(format!("{}=trace", module_path!())),
    )?;

    Ok(())
}
