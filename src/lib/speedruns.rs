//! Tools to download, search, and mirror https://speedrun.com leaderboards.
#![feature(
    arbitrary_self_types,
    label_break_value,
    option_unwrap_none,
    never_type
)]
#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(missing_debug_implementations)]
#![deny(unconditional_recursion)]

/// Types for the speedrun.com API data we consume and utilities for normalizing it.  
pub mod api;
/// Our normalized data types, a frozen in-memory database, and leaderboard logic.
pub mod data;
/// Utilities that should probably go somewhere more specific.
pub mod utils;

pub use crate::data::{
    database::Database,
    types::{self, *},
};
