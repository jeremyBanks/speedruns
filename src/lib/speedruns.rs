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
pub mod database {
    use std::collections::{BTreeMap as SortedMap, HashMap};
    use std::sync::Arc;

    use crate::types::{Category, Game, Level, Run, User};

    #[derive(Debug, Clone)]
    pub struct Tables {
        games: HashMap<u64, Game>,
        categories: HashMap<u64, Category>,
        runs: HashMap<u64, Run>,
        users: HashMap<u64, User>,
        levels: HashMap<u64, Level>,
    }

    #[derive(Debug, Clone)]
    pub struct Indicies<'tables> {
        tables: &'tables Tables,
        games_by_slug: SortedMap<String, &'tables Game>,
    }

    impl<'tables> Indicies<'tables> {
        pub fn from_tables<'a>(tables: &'a Tables) -> Indicies<'a> {
            Indicies {
                tables,
                games_by_slug: tables
                    .games
                    .values()
                    .map(|game| (game.slug.clone(), game))
                    .collect(),
            }
        }
    }

    rental! {
        mod rentals {
            use super::*;
            #[rental(debug, clone, covariant)]
            pub struct Database {
                tables: Arc<Tables>,
                indicies: Indicies<'tables>,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Database(rentals::Database);

    impl Database {
        pub fn new(tables: Arc<Tables>) -> Database {
            Database(rentals::Database::new(tables, |tables| {
                Indicies::from_tables(tables)
            }))
        }

        pub fn tables(&self) -> &Tables {
            self.0.head()
        }

        pub fn indicies(&self) -> &Indicies {
            self.0.suffix()
        }
    }
}

/// Functions for leaderboards, progression, etc.
pub mod aggregation {
    pub use super::data::leaderboard;
    pub use super::data::progression;
}

/// Utilities that should probably go somewhere more specific.
pub use speedruns_utils as utils;

pub use crate::data::{database::Database, types::*};
