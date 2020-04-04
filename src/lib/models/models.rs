#![allow(missing_docs)]
use std::{
    cmp::{Eq, Ord},
    convert::From,
};

use chrono::{DateTime, NaiveDate, Utc};
use getset::Getters;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};
use validator_derive::Validate;

use speedruns_utils::base36;

pub mod aggregation;
pub mod any;

// We currently represent all ids as u64s for efficiency.
// You can use [crate::utils] to convert to and from speedrun.com's
// API IDs. (This isn't the same conversion as speedrun.com uses,
// so the ordering of these IDs doesn't align with insertion time
// or anything like that.)

// The .validate() methods on each record are sanity checks.
// It shouldn't be possible for imported records to fail validation.
// If they do, that's a bug.
//
// TODO: Make our own validation trait, and move all of the validation
// logic to a different module.

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Hash,
    PartialOrd,
    Ord,
    Eq,
    Getters,
    Validate,
)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct Category {
    pub game_id: u64,
    #[validate(length(min = 1))]
    pub slug: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub id: u64,
    pub per: CategoryType,
    pub rules: String,
}

impl Category {
    /// This item's ID as it would be formatted for speedrun.com.
    pub fn src_id(&self) -> String {
        base36(*self.id())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
#[serde(deny_unknown_fields)]
pub enum CategoryType {
    PerGame,
    PerLevel,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Hash,
    PartialOrd,
    Ord,
    Eq,
    Getters,
    Validate,
)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct User {
    pub created: Option<DateTime<Utc>>,
    #[validate(length(min = 1))]
    pub slug: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub id: u64,
}

impl User {
    /// This item's ID as it would be formatted for speedrun.com.
    pub fn src_id(&self) -> String {
        base36(*self.id())
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Hash,
    PartialOrd,
    Ord,
    Eq,
    Getters,
    Validate,
)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct Game {
    pub id: u64,
    pub created: Option<DateTime<Utc>>,
    #[validate(length(min = 1))]
    pub slug: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub primary_timing: TimingMethod,
}

impl Game {
    /// This item's ID as it would be formatted for speedrun.com.
    pub fn src_id(&self) -> String {
        base36(*self.id())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum TimingMethod {
    IGT,
    RTA,
    RTA_NL,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Hash,
    PartialOrd,
    Ord,
    Eq,
    Getters,
    Validate,
)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct Level {
    pub game_id: u64,
    pub id: u64,
    #[validate(length(min = 1))]
    pub slug: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub rules: String,
}

impl Level {
    /// This item's ID as it would be formatted for speedrun.com.
    pub fn src_id(&self) -> String {
        base36(*self.id())
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Hash,
    Clone,
    PartialOrd,
    Ord,
    Eq,
    Getters,
    Validate,
)]
#[get = "pub"]
pub struct Run {
    pub game_id: u64,
    pub category_id: u64,
    pub level_id: Option<u64>,
    pub id: u64,
    pub created: Option<DateTime<Utc>>,
    pub date: Option<NaiveDate>,
    #[validate]
    pub times_ms: RunTimesMs,
    #[validate]
    pub players: Vec<RunPlayer>,
    pub videos: Vec<RunVideo>,
}

impl Run {
    /// This item's ID as it would be formatted for speedrun.com.
    pub fn src_id(&self) -> String {
        base36(*self.id())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
#[serde(deny_unknown_fields)]
pub enum RunVideo {
    YouTube { id: String, start: Option<i32> },
    Link { url: String },
}

impl std::fmt::Display for RunVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use RunVideo::*;
        match self {
            YouTube { id, start } => {
                write!(f, "https://youtu.be/{}?t={}", id, start.unwrap_or(0))
            }
            Link { url } => write!(f, "{}", url),
        }
    }
}

impl std::str::FromStr for RunVideo {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: parse all youtube URL variants
        Ok(RunVideo::Link { url: s.to_string() })
    }
}

#[derive(
    Debug, Serialize, Deserialize, PartialEq, Hash, Clone, PartialOrd, Ord, Eq, Getters,
)]
#[serde(deny_unknown_fields)]
#[get = "pub"]
pub struct RunTimesMs {
    pub igt: Option<u64>,
    pub rta: Option<u64>,
    pub rta_nl: Option<u64>,
}

impl RunTimesMs {
    pub fn get(&self, timing: &TimingMethod) -> Option<u64> {
        match timing {
            TimingMethod::IGT => *self.igt(),
            TimingMethod::RTA => *self.rta(),
            TimingMethod::RTA_NL => *self.rta_nl(),
        }
    }
}

impl Validate for RunTimesMs {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.igt == None && self.rta == None && self.rta_nl == None {
            let mut errors = ValidationErrors::new();
            errors.add("", ValidationError::new("all times were None"));
            return Err(errors);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, PartialOrd, Ord, Eq)]
#[serde(deny_unknown_fields)]
pub enum RunPlayer {
    UserId(u64),
    GuestName(String),
}

impl Validate for RunPlayer {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if let RunPlayer::GuestName(name) = self {
            if name.is_empty() {
                let mut errors = ValidationErrors::new();
                errors.add("GuestName.0", ValidationError::new("name is empty"));
                return Err(errors);
            }
        }
        Ok(())
    }
}
