use std::collections::BTreeMap;
use std::collections::{BTreeMap as SortedMap, HashMap, HashSet};
use std::{hash::Hash, sync::Arc};

use chrono::{DateTime, NaiveDateTime, Utc};
use getset::Getters;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use log::{debug, error, info, trace, warn};

use speedruns_models::{Category, CategoryType, Game, Level, Run, User};

#[macro_use]
extern crate rental;

mod integrity;
pub use integrity::{validate, IntegrityError, IntegrityErrors};

#[derive(Debug, Clone)]
pub struct Database(rentals::Database);

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

impl Tables {
    pub fn new(
        games: impl IntoIterator<Item = Game>,
        categories: impl IntoIterator<Item = Category>,
        levels: impl IntoIterator<Item = Level>,
        runs: impl IntoIterator<Item = Run>,
        users: impl IntoIterator<Item = User>,
    ) -> Tables {
        Tables {
            games: games.into_iter().map(|x| (*x.id(), x)).collect(),
            categories: categories.into_iter().map(|x| (*x.id(), x)).collect(),
            runs: runs.into_iter().map(|x| (*x.id(), x)).collect(),
            users: users.into_iter().map(|x| (*x.id(), x)).collect(),
            levels: levels.into_iter().map(|x| (*x.id(), x)).collect(),
        }
    }
}

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
                    let mut invalid_games = HashSet::<Game>::new();
                    let mut invalid_categories = HashSet::<Category>::new();
                    let mut invalid_levels = HashSet::<Level>::new();
                    let mut invalid_runs = HashSet::<Run>::new();
                    let mut invalid_users = HashSet::<User>::new();

                    for error in errors.errors {
                        let invalid_rows = error.invalid_rows();
                        invalid_games.extend(invalid_rows.games);
                        invalid_categories.extend(invalid_rows.categories);
                        invalid_levels.extend(invalid_rows.levels);
                        invalid_runs.extend(invalid_rows.runs);
                        invalid_users.extend(invalid_rows.users);
                    }

                    fn filter_invalid<T: Hash + Eq + Clone>(
                        table: &HashMap<u64, T>,
                        invalid: HashSet<T>,
                    ) -> HashMap<u64, T> {
                        table
                            .iter()
                            .filter(|(_id, row)| !invalid.contains(row))
                            .map(|(id, row)| (id.clone(), T::clone(row)))
                            .collect()
                    }

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
                        (invalid_categories.len() * 100) / tables.categories().len().max(1)
                    );
                    error!(
                        "{:6} ({:3}%) invalid levels",
                        invalid_levels.len(),
                        (invalid_levels.len() * 100) / tables.levels().len().max(1)
                    );

                    tables = Arc::new(Tables {
                        games: filter_invalid(&tables.games, invalid_games),
                        categories: filter_invalid(&tables.categories, invalid_categories),
                        levels: filter_invalid(&tables.levels, invalid_levels),
                        runs: filter_invalid(&tables.runs, invalid_runs),
                        users: filter_invalid(&tables.users, invalid_users),
                    })
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
    fn tables(&self) -> &Tables {
        self.0.head()
    }

    /// With indicies of the database, with references to rows tree-indexed
    /// in different ways, and some aggregated values.
    pub fn indicies(&self) -> &Indicies {
        self.0.suffix()
    }

    pub fn games(&self) -> &HashMap<u64, Game> {
        self.tables().games()
    }

    pub fn categories(&self) -> &HashMap<u64, Category> {
        self.tables().categories()
    }

    pub fn levels(&self) -> &HashMap<u64, Level> {
        self.tables().levels()
    }

    pub fn runs(&self) -> &HashMap<u64, Run> {
        self.tables().runs()
    }

    pub fn users(&self) -> &HashMap<u64, User> {
        self.tables().users()
    }
}

impl<'tables> Indicies<'tables> {
    pub fn from_tables(tables: &'tables Tables) -> Indicies<'tables> {
        /// Index rows by some unique key. (Uniqueness not validated.)
        fn index<
            'tables,
            Value,
            OldKey: 'tables + Hash + Eq,
            NewKey: 'tables + Ord + Eq,
        >(
            original: &'tables HashMap<OldKey, Value>,
            key: fn(&'tables Value) -> NewKey,
        ) -> BTreeMap<NewKey, &'tables Value> {
            index_where(original, key, |_| true)
        }

        /// Index rows passing some filter by some unique key. (Uniqueness not validated.)
        fn index_where<'tables, Value, OldKey: 'tables + Hash + Eq, NewKey: Ord + Eq>(
            original: &'tables HashMap<OldKey, Value>,
            key: fn(&'tables Value) -> NewKey,
            filter: fn(&Value) -> bool,
        ) -> BTreeMap<NewKey, &'tables Value> {
            // TODO: Should this use rayon?
            original
                .values()
                .filter(|x| filter(x))
                .map(|value| (key(value), value))
                .collect()
        }

        /// Index groups of rows by some non-unique key.
        fn index_group<
            'tables,
            Value,
            OldKey: 'tables + Hash + Eq,
            NewKey: 'tables + Ord + Eq,
        >(
            original: &HashMap<OldKey, Value>,
            key: fn(&'tables Value) -> NewKey,
        ) -> BTreeMap<NewKey, &'tables Value> {
            unimplemented!(
                "these lifetimes confuse me

            original
                .values()
                .group_by(|x| key(x))
                .into_iter()
                .map(|(key, values)| (key, values.collect()))
                .collect()"
            );
        }

        Indicies {
            last_updated: tables
                .runs()
                .values()
                .map(|run| run.created)
                .flatten()
                .max()
                .unwrap_or(DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(0, 0),
                    Utc,
                )),

            games_by_slug: index(tables.games(), |game| game.slug().as_ref()),

            users_by_slug: index(tables.users(), |user| user.slug().as_ref()),

            levels_by_game_id_and_slug: index(tables.levels(), |level| {
                (*level.game_id(), level.slug().as_ref())
            }),

            per_game_categories_by_game_id_and_slug: index_where(
                tables.categories(),
                |category| (*category.game_id(), category.slug().as_ref()),
                |category| *category.per() == CategoryType::PerGame,
            ),

            per_level_categories_by_game_id_and_slug: index_where(
                tables.categories(),
                |category| (*category.game_id(), category.slug().as_ref()),
                |category| *category.per() == CategoryType::PerLevel,
            ),

            runs_by_game_id_and_category_id_and_level_id: tables
                .runs()
                .values()
                .group_by(|run| (*run.game_id(), *run.category_id(), *run.level_id()))
                .into_iter()
                .map(|(key, runs)| (key, runs.collect()))
                .collect(),
        }
    }
}
pub trait TableUtils {}

impl<Row> TableUtils for HashMap<u64, Row> {}
pub trait IndexUtils {}

impl<Key: Ord + Eq, RowRef> IndexUtils for BTreeMap<Key, RowRef> {}
