//! Tools to download, search, and mirror https://speedrun.com leaderboards.
#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(missing_debug_implementations)]

#[macro_use]
extern crate rental;

/// Types for the speedrun.com API data we consume, and utilities for normalizing it.  
pub mod api;

/// Our normalized data types, a frozen in-memory database, and leaderboard logic.
///
/// TODO: refactor this out of existence
pub mod data;

/// The core types of our data model.
pub mod types;

/// Validating, indexing, and serializing our data.
pub mod database;

/// Functions for leaderboards, progression, etc.
pub mod aggregation {
    pub use super::data::leaderboard;
    pub use super::data::progression;
}

/// Utilities that should probably go somewhere more specific.
pub use speedruns_utils as utils;

pub use crate::data::{database::Database, types::*};
