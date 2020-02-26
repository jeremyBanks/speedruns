//! Dynamic types for our core data models.
use std::{convert::TryFrom, fmt::Debug, hash::Hash};

use chrono::{DateTime, Utc};
use derive_more::{From, TryInto};
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::*;

/// A core type we store in a Database.
pub trait Model:
    Into<AnyModel>
    + TryFrom<AnyModel>
    + Debug
    + Serialize
    + DeserializeOwned
    + Clone
    + PartialEq
    + Hash
    + PartialOrd
    + Ord
    + Eq
{
    fn id(&self) -> u64;
    fn created(&self) -> Option<DateTime<Utc>>;
}

/// Any Model type.
#[derive(
    From,
    TryInto,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Hash,
    PartialOrd,
    Ord,
    Eq,
)]
pub enum AnyModel {
    Run(Run),
    User(User),
    Game(Game),
    Category(Category),
    Level(Level),
}

/// A reference to a homogenous Vec of any Model type.
#[derive(
    From,
    TryInto,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Hash,
    PartialOrd,
    Ord,
    Eq,
)]
pub enum AnyModelVec {
    Runs(Vec<Run>),
    Users(Vec<User>),
    Games(Vec<Game>),
    Categories(Vec<Category>),
    Levels(Vec<Level>),
}

impl Model for AnyModel {
    fn id(&self) -> u64 {
        match self {
            AnyModel::Run(run) => Model::id(run),
            AnyModel::User(user) => Model::id(user),
            AnyModel::Game(game) => Model::id(game),
            AnyModel::Category(category) => Model::id(category),
            AnyModel::Level(level) => Model::id(level),
        }
    }

    fn created(&self) -> Option<DateTime<Utc>> {
        match self {
            AnyModel::Run(run) => Model::created(run),
            AnyModel::User(user) => Model::created(user),
            AnyModel::Game(game) => Model::created(game),
            AnyModel::Category(category) => Model::created(category),
            AnyModel::Level(level) => Model::created(level),
        }
    }
}

impl Model for Run {
    fn id(&self) -> u64 {
        *Run::id(self)
    }

    fn created(&self) -> Option<DateTime<Utc>> {
        *Run::created(self)
    }
}

impl Model for User {
    fn id(&self) -> u64 {
        *User::id(self)
    }

    fn created(&self) -> Option<DateTime<Utc>> {
        *User::created(self)
    }
}

impl Model for Game {
    fn id(&self) -> u64 {
        *Game::id(self)
    }

    fn created(&self) -> Option<DateTime<Utc>> {
        *Game::created(self)
    }
}

impl Model for Category {
    fn id(&self) -> u64 {
        *Category::id(self)
    }

    fn created(&self) -> Option<DateTime<Utc>> {
        None
    }
}

impl Model for Level {
    fn id(&self) -> u64 {
        *Level::id(self)
    }

    fn created(&self) -> Option<DateTime<Utc>> {
        None
    }
}
