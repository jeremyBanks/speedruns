//! Tools to download, search, and mirror https://speedrun.com leaderboards.
#![feature(
    arbitrary_self_types,
    associated_type_defaults,
    proc_macro_hygiene,
    label_break_value,
    slice_concat_ext
)]
#![allow(missing_docs, clippy::useless_attribute)]
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
