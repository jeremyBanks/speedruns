//! A simplified and normalized data model, with shared data referenced by ID.
//!
//! This doesn't include all of the metadata from speedrun.com, and excludes
//! corrupt records and rejected or pending runs.
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

use crate::validators::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Getters, Validate)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct User {
    pub id: p64,
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Getters, Validate)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct Game {
    pub id: p64,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Getters, Validate)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct Category {
    pub id: p64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Getters, Validate)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct Level {
    pub id: p64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Hash, Clone, Getters, Validate)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct Run {
    pub id: p64,
    pub created: Option<DateTime<Utc>>,
    pub game_id: p64,
    /* level_id: Option<p64>,
    pub primary_time_ms: u64,
     * category_id: p64,
     * players: Vec<RunPlayer>,
     * date: Option<NaiveDate>,
     * comment: String,
     * #[validate(custom = "urls")]
     * video_urls: Vec<String>,
     * #[validate(custom = "urls")]
     * split_urls: Vec<String>, */
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub enum RunPlayer {
    UserId(p64),
    GuestName(String),
}

impl Validate for RunPlayer {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if let RunPlayer::GuestName(name) = self {
            if name.len() < 1 {
                let mut errors = ValidationErrors::new();
                errors.add("GuestName.0", ValidationError::new("name is empty"));
                return Err(errors);
            }
        }
        Ok(())
    }
}
