//! The world's worst in-memory database of normalized speedrun data.
#![warn(missing_debug_implementations, missing_docs)]
#![allow(missing_debug_implementations, missing_docs)]
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    num::NonZeroU64 as Id64,
};

use getset::Getters;
use lazy_init::Lazy;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::data::{linked::Linked, types::*};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Getters)]
#[get = "pub"]
pub struct Database {
    runs:       BTreeMap<Id64, Run>,
    users:      BTreeMap<Id64, User>,
    games:      BTreeMap<Id64, Game>,
    categories: BTreeMap<Id64, Category>,
    levels:     BTreeMap<Id64, Level>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn indices(&self) -> Indices {
        Indices::new(self)
    }

    pub fn insert_game(&mut self, game: Game) {
        self.games.insert(*game.id(), game);
    }

    pub fn insert_user(&mut self, user: User) {
        self.users.insert(*user.id(), user);
    }

    pub fn insert_run(&mut self, run: Run) {
        self.runs.insert(*run.id(), run);
    }

    pub fn insert_level(&mut self, level: Level) {
        self.levels.insert(*level.id(), level);
    }

    pub fn insert_category(&mut self, category: Category) {
        self.categories.insert(*category.id(), category);
    }

    /// Generates an index mapping Games to sorted lists of Runs.
    pub fn runs_by_game_id(&self) -> HashMap<Id64, Vec<&Run>> {
        info!("Indexing runs by game id...");
        let mut index = HashMap::new();

        for game_id in self.games().keys() {
            index.insert(*game_id, vec![]);
        }

        for run in self.runs().values() {
            index.get_mut(run.game_id()).unwrap().push(run);
        }

        for game_runs in index.values_mut() {
            game_runs.sort();
        }

        index
    }

    /// Generates an index mapping Games to sorted lists of Runs.
    pub fn games_by_slug(&self) -> HashMap<&str, &Game> {
        info!("Indexing games by slug...");
        let mut index: HashMap<&str, &Game> = HashMap::new();

        for game in self.games().values() {
            index.insert(game.slug(), game);
        }

        index
    }

    /// Ranks a set of runs (all for the same game/category/level) using the
    /// timing specified for the game rules, then by run date, then by
    /// submission datetime.
    pub fn rank_runs<'db>(&'db self, runs: &[&'db Run]) -> Vec<RankedRun> {
        let mut runs: Vec<&Run> = runs.to_vec();

        if runs.is_empty() {
            return vec![]
        }

        let first = runs[0];
        let game = self
            .games()
            .get(first.game_id())
            .expect("game should exist");

        runs.sort_by_key(|run| {
            let time_ms = run.times_ms().get(game.primary_timing()).unwrap();

            (time_ms, run.date(), run.created())
        });

        let mut ranks: Vec<RankedRun> = vec![];

        for (i, run) in runs.iter().enumerate() {
            assert_eq!(run.game_id(), first.game_id());
            assert_eq!(run.level_id(), first.level_id());
            assert_eq!(run.category_id(), first.category_id());

            let time_ms = run.times_ms().get(game.primary_timing()).unwrap();
            let rank = Id64::new((i + 1) as u64).unwrap();
            let mut tied_rank = rank;
            let mut is_tied = false;

            if let Some(ref mut previous) = ranks.last_mut() {
                if time_ms == *previous.time_ms() {
                    is_tied = true;
                    previous.is_tied = true;
                    tied_rank = previous.tied_rank;
                }
            }

            let new = RankedRun {
                rank,
                time_ms,
                is_tied,
                tied_rank,
                run,
            };

            ranks.push(new);
        }

        ranks
    }
}

pub struct Indices<'db> {
    database: &'db Database,
    runs_by_game_id: Lazy<BTreeMap<Id64, Vec<Linked<'db, Run>>>>,
}

impl<'db> Indices<'db> {
    pub fn new(database: &'db Database) -> Self {
        Self {
            database,
            runs_by_game_id: Lazy::new(),
        }
    }

    pub fn game(&self, id: Id64) -> Linked<'db, Game> {
        Linked::new(&self, self.database.games.get(&id).expect("foreign key to be valid"))
    }

    pub fn runs_by_game_id(&self) -> &BTreeMap<Id64, Vec<Linked<'db, Run>>> {
        self.runs_by_game_id.get_or_create(|| {
            self.database.runs().len();
            BTreeMap::new()
        })
    }
}

#[derive(Debug, Clone, Getters, Serialize)]
#[get = "pub"]
pub struct RankedRun<'db> {
    rank:      Id64,
    time_ms:   u64,
    is_tied:   bool,
    tied_rank: Id64,
    run:       &'db Run,
}

impl Validate for Database {
    fn validate(&self) -> Result<(), ValidationErrors> {
        fn validate_table<T: Validate + Debug>(
            table: &BTreeMap<Id64, T>,
        ) -> Result<(), ValidationErrors> {
            for item in table.values() {
                let result = item.validate();
                if let Err(ref error) = result {
                    error!("{} in {:?}", &error, &item);
                }
                result?;
            }
            Ok(())
        }

        // TODO:
        // foreign keys
        // unique constraints
        // indexed by id

        info!("Validating {} normalized games...", self.games().len());
        validate_table(self.games())?;
        info!("Validating {} normalized users...", self.users().len());
        validate_table(self.users())?;
        info!("Validating {} normalized runs...", self.runs().len());
        validate_table(self.runs())?;
        info!("Validating {} normalized levels...", self.levels().len());
        validate_table(self.levels())?;
        info!(
            "Validating {} normalized categories...",
            self.categories().len()
        );
        validate_table(self.categories())?;

        Ok(())
    }
}
