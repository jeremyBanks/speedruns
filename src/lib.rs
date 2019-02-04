//! Tools to download, search, and mirror https://speedrun.com leaderboards.
#![feature(associated_type_defaults, proc_macro_hygiene, label_break_value)]
// #![warn(missing_debug_implementations, missing_docs)]
#![allow(clippy::useless_attribute)]

pub mod api;
pub mod data;
pub mod server;
pub mod utils;

pub use crate::data::{
    base::Database,
    types::{self, *},
};
