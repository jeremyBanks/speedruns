//! Tools to download, search, and mirror https://speedrun.com leaderboards.
#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(missing_debug_implementations)]

/// Types for the speedrun.com API data we consume and utilities for normalizing it.  
pub mod api;

/// Our normalized data types, a frozen in-memory database, and leaderboard logic.
pub mod data;

/// Utilities that should probably go somewhere more specific.
pub use speedruns_utils as utils;

pub use crate::data::{
    database::Database,
    types::{self, *},
};
