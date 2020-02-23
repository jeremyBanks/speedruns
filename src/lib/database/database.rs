use std::collections::{BTreeMap as SortedMap, HashMap, HashSet};
use std::sync::Arc;

use chrono::{DateTime, NaiveDateTime, Utc};
use getset::Getters;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use crate::types::{Category, CategoryType, Game, Level, Run, User};

use super::integrity::{validate, IntegrityErrors};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Getters)]
#[get = "pub"]
pub struct Tables {
    games: HashMap<u64, Game>,
    categories: HashMap<u64, Category>,
    runs: HashMap<u64, Run>,
    users: HashMap<u64, User>,
    levels: HashMap<u64, Level>,
}

#[derive(Debug, Clone, Getters)]
#[get = "pub"]
pub struct Indicies<'tables> {
    last_updated: DateTime<Utc>,
    games_by_slug: SortedMap<&'tables str, &'tables Game>,
    users_by_slug: SortedMap<&'tables str, &'tables User>,
    per_game_categories_by_game_id_and_slug:
        SortedMap<(u64, &'tables str), &'tables Category>,
    per_level_categories_by_game_id_and_slug:
        SortedMap<(u64, &'tables str), &'tables Category>,
    levels_by_game_id_and_slug: SortedMap<(u64, &'tables str), &'tables Level>,
    runs_by_game_id_and_category_id_and_level_id:
        SortedMap<(u64, u64, Option<u64>), Vec<&'tables Run>>,
}

impl<'tables> Indicies<'tables> {
    pub fn from_tables(tables: &'tables Tables) -> Indicies<'tables> {
        // Repeatedly iterating like this is slower but the code's simpler.
        Indicies {
            last_updated: tables
                .runs
                .values()
                .map(|run| run.created)
                .flatten()
                .max()
                .unwrap_or(DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(0, 0),
                    Utc,
                )),

            games_by_slug: tables
                .games
                .values()
                .map(|game| (game.slug().as_ref(), game))
                .collect(),

            users_by_slug: tables
                .users
                .values()
                .map(|user| (user.slug().as_ref(), user))
                .collect(),

            levels_by_game_id_and_slug: tables
                .levels
                .values()
                .map(|level| ((*level.game_id(), level.slug().as_ref()), level))
                .collect(),

            per_game_categories_by_game_id_and_slug: tables
                .categories
                .values()
                .filter(|category| *category.per() == CategoryType::PerGame)
                .map(|category| ((*category.game_id(), category.slug().as_ref()), category))
                .collect(),

            per_level_categories_by_game_id_and_slug: tables
                .categories
                .values()
                .filter(|category| *category.per() == CategoryType::PerLevel)
                .map(|category| ((*category.game_id(), category.slug().as_ref()), category))
                .collect(),

            runs_by_game_id_and_category_id_and_level_id: tables
                .runs
                .values()
                .group_by(|run| (*run.game_id(), *run.category_id(), *run.level_id()))
                .into_iter()
                .map(|(key, runs)| (key, runs.collect()))
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
    /// Initialize a Database from table data.
    ///
    /// If any data fails validation, the tables will be cloned with
    /// validation-failing rows filtered out.
    pub fn new(tables: Arc<Tables>) -> Database {
        let mut tables = tables;
        loop {
            match Self::try_new(tables.clone()) {
                Ok(self_) => return self_,
                Err(errors) => {
                    let mut invalid_runs = HashSet::<&Run>::new();
                    let mut invalid_games = HashSet::<&Game>::new();
                    let mut invalid_users = HashSet::<&User>::new();
                    let mut invalid_levels = HashSet::<&Level>::new();
                    let mut invalid_categories = HashSet::<&Category>::new();

                    for error in errors.errors {
                        let invalid_rows = errors.invalid_rows();
                        invalid_runs.extend(invalid_rows.runs);
                        invalid_runs.extend(invalid_rows.runs);
                        invalid_runs.extend(invalid_rows.runs);
                        invalid_runs.extend(invalid_rows.runs);
                        invalid_runs.extend(invalid_rows.runs);

                        error!(
                            "{:6} ({:3}%) invalid runs",
                            invalid_runs.len(),
                            (invalid_runs.len() * 100) / tables.runs().len().max(1)
                        );
                        error!(
                            "{:6} ({:3}%) invalid users",
                            invalid_users.len(),
                            (invalid_users.len() * 100) / tables.users().len().max(1)
                        );
                        error!(
                            "{:6} ({:3}%) invalid games",
                            invalid_games.len(),
                            (invalid_games.len() * 100) / tables.games().len().max(1)
                        );
                        error!(
                            "{:6} ({:3}%) invalid categories",
                            invalid_categories.len(),
                            (invalid_categories.len() * 100)
                                / tables.categories().len().max(1)
                        );
                        error!(
                            "{:6} ({:3}%) invalid levels",
                            invalid_levels.len(),
                            (invalid_levels.len() * 100) / tables.levels().len().max(1)
                        );

                        tables = Arc::new(Tables {
                            games: tables
                                .games()
                                .iter()
                                .filter(|(_id, game)| !invalid_games.contains(game))
                                .map(|(id, game)| (id.clone(), game.clone()))
                                .collect(),
                            categories: tables
                                .categories()
                                .iter()
                                .filter(|(_id, category)| {
                                    !invalid_categories.contains(category)
                                })
                                .map(|(id, category)| (id.clone(), category.clone()))
                                .collect(),
                            levels: tables
                                .levels()
                                .iter()
                                .filter(|(_id, level)| !invalid_levels.contains(level))
                                .map(|(id, level)| (id.clone(), level.clone()))
                                .collect(),
                            runs: tables
                                .runs()
                                .iter()
                                .filter(|(_id, run)| !invalid_runs.contains(run))
                                .map(|(id, run)| (id.clone(), run.clone()))
                                .collect(),
                            users: tables
                                .users()
                                .iter()
                                .filter(|(_id, user)| !invalid_users.contains(user))
                                .map(|(id, user)| (id.clone(), user.clone()))
                                .collect(),
                        })
                    }
                }
            }
        }
    }

    /// Attempt to initialize a Database from table data.
    ///
    /// If any data fails validation, this will return an Err of
    /// IntegrityErrors indicating the records that caused the failure.
    pub fn try_new(tables: Arc<Tables>) -> Result<Database, IntegrityErrors> {
        let self_ = Self::new_unvalidated(tables);
        validate(&self_).map(move |()| self_)
    }

    /// Initialize a Database from table data, assuming it to be valid.
    fn new_unvalidated(tables: Arc<Tables>) -> Database {
        Database(rentals::Database::new(tables, |tables| {
            Indicies::from_tables(tables)
        }))
    }

    /// The tables of the database, with all rows hash-indexed by ID.
    pub fn tables(&self) -> &Tables {
        self.0.head()
    }

    /// With indicies of the database, with references to rows tree-indexed
    /// in different ways, and some aggregated values.
    pub fn indicies(&self) -> &Indicies {
        self.0.suffix()
    }
}
