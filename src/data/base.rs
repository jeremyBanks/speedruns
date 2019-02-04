//! The world's worst in-memory database of normalized speedrun data.
#![warn(missing_debug_implementations, missing_docs)]
#![allow(missing_debug_implementations, missing_docs)]
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Debug, Display},
    num::NonZeroU64 as Id64,
    ops::Deref,
    rc::Rc,
};

use derive_more::From;
use err_derive::Error;
use getset::Getters;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::{data::types::*, utils::base36};

#[derive(Debug, Error, From)]
pub struct IntegrityErrors {
    pub errors: Vec<IntegrityError>,
}

impl IntegrityErrors {
    fn try_from(errors: Vec<IntegrityError>) -> Result<(), IntegrityErrors> {
        if errors.is_empty() {
            Ok(())
        } else {
            Err(IntegrityErrors { errors })
        }
    }
}

impl Display for IntegrityErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{} IntegrityErrors:", self.errors.len())?;
        for (i, error) in self.errors.iter().enumerate() {
            writeln!(f, "{:4}. {}", i + 1, error)?;
            if i >= 3 {
                writeln!(f, "     ...and more!")?;
                break
            }
        }
        Ok(())
    }
}

#[derive(Debug, Error, From)]
pub enum IntegrityError {
    #[error(
        display = "{} with id {} does not exist, specified by {} of {} {} in {:#?}",
        target_type,
        target_id,
        foreign_key_field,
        source_type,
        source_id,
        source
    )]
    ForeignKeyMissing {
        source:            Box<dyn Debug>,
        source_type:       &'static str,
        source_id:         Id64,
        foreign_key_field: &'static str,
        target_id:         Id64,
        target_type:       &'static str,
    },
    #[error(display = "row validation check failed: {:?} in {:?}", errors, item)]
    CheckFailed {
        item:   Box<dyn Debug>,
        errors: ValidationErrors,
    },
    #[error(display = "indexing failed, expect a ForeignKeyMissing saying why")]
    IndexingError,
}

/// All of the speedrun data in our normalized format, indexed by ID.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Getters)]
#[get = "pub"]
pub struct Tables {
    runs:       BTreeMap<Id64, Run>,
    users:      BTreeMap<Id64, User>,
    games:      BTreeMap<Id64, Game>,
    categories: BTreeMap<Id64, Category>,
    levels:     BTreeMap<Id64, Level>,
}

/// Marker trait for types that we store in [Tables].
pub trait Model: Debug + Serialize + Validate {}
impl Model for Run {}
impl Model for User {}
impl Model for Game {}
impl Model for Category {}
impl Model for Level {}

impl Tables {
    pub fn new(
        runs: Vec<Run>,
        users: Vec<User>,
        games: Vec<Game>,
        categories: Vec<Category>,
        levels: Vec<Level>,
    ) -> Self {
        let mut self_ = Self::default();
        for run in runs {
            self_.runs.insert(*run.id(), run);
        }
        for user in users {
            self_.users.insert(*user.id(), user);
        }
        for game in games {
            self_.games.insert(*game.id(), game);
        }
        for category in categories {
            self_.categories.insert(*category.id(), category);
        }
        for level in levels {
            self_.levels.insert(*level.id(), level);
        }
        self_
    }
}

/// Panic message used when the database state is invalid but that shouldn't be
/// possible, because it must have alrady been validated, such as for foreign
/// key lookups.
const DATABASE_UNEXPECTEDLY_INVALID: &str =
    "Database state invalid despite passing validation?!";

/// A collection of [Tables] with various generated indexes.
pub struct Database {
    tables:                         &'static Tables,
    runs_by_game_id:                HashMap<Id64, Vec<&'static Run>>,
    games_by_slug:                  HashMap<&'static str, &'static Game>,
    users_by_name:                  HashMap<&'static str, &'static User>,
    categories_by_game_id_and_name: HashMap<(Id64, &'static str), &'static Category>,
    levels_by_game_id_and_name:     HashMap<(Id64, &'static str), &'static Level>,
}

impl Database {
    fn link<ModelType: Model>(
        self: Rc<Self>,
        item: &'static ModelType,
    ) -> Linked<ModelType> {
        Linked::new(self.clone(), item)
    }

    /// Creates a new Database indexing a collection of static tables.
    pub fn new(tables: &'static Tables) -> Result<Rc<Self>, IntegrityErrors> {
        let mut runs_by_game_id: HashMap<Id64, Vec<&'static Run>> = HashMap::new();
        let mut games_by_slug: HashMap<&'static str, &'static Game> = HashMap::new();
        let mut users_by_name: HashMap<&'static str, &'static User> = HashMap::new();
        let mut categories_by_game_id_and_name: HashMap<
            (Id64, &'static str),
            &'static Category,
        > = HashMap::new();
        let mut levels_by_game_id_and_name: HashMap<(Id64, &'static str), &'static Level> =
            HashMap::new();

        let index_errored = 'indexing: {
            for game in tables.games().values() {
                runs_by_game_id.insert(*game.id(), Vec::new());
                games_by_slug.insert(game.slug(), game);
            }

            for run in tables.runs().values() {
                if let Some(runs) = runs_by_game_id.get_mut(run.game_id()) {
                    runs.push(run);
                } else {
                    break 'indexing true
                }
            }

            for user in tables.users().values() {
                users_by_name.insert(user.name(), user);
            }

            for category in tables.categories().values() {
                categories_by_game_id_and_name
                    .insert((*category.game_id(), category.name()), category);
            }

            for level in tables.levels().values() {
                levels_by_game_id_and_name.insert((*level.game_id(), level.name()), level);
            }

            for game_runs in runs_by_game_id.values_mut() {
                game_runs.sort();
            }

            false
        };

        let mut errors = Vec::new();
        if index_errored {
            error!("indexing failed, database must have validity errors");
            errors.push(IntegrityError::IndexingError)
        }

        let self_ = Rc::new(Self {
            tables,
            runs_by_game_id,
            games_by_slug,
            users_by_name,
            categories_by_game_id_and_name,
            levels_by_game_id_and_name,
        });

        if let Err(mut errors_) = self_.clone().validate() {
            errors.append(&mut errors_.errors);
        }

        IntegrityErrors::try_from(errors).map(|_| self_)
    }

    pub fn validate(self: Rc<Self>) -> Result<(), IntegrityErrors> {
        let mut errors = vec![];

        trace!("Validating {} runs.", self.tables.runs().len());
        for run in self.clone().runs() {
            if let Err(mut error) = run.validate() {
                errors.append(&mut error.errors);
            }
        }

        trace!("Validating {} users.", self.tables.users().len());
        for user in self.clone().users() {
            if let Err(mut error) = user.validate() {
                errors.append(&mut error.errors);
            }
        }

        trace!("Validating {} games.", self.tables.games().len());
        for game in self.clone().games() {
            if let Err(mut error) = game.validate() {
                errors.append(&mut error.errors);
            }
        }

        trace!("Validating {} categories.", self.tables.categories().len());
        for category in self.clone().categories() {
            if let Err(mut error) = category.validate() {
                errors.append(&mut error.errors);
            }
        }

        trace!("Validating {} levels.", self.tables.levels().len());
        for level in self.clone().levels() {
            if let Err(mut error) = level.validate() {
                errors.append(&mut error.errors);
            }
        }

        IntegrityErrors::try_from(errors)
    }

    /// Iterator over all Linked<Run>s.
    pub fn runs(self: Rc<Self>) -> impl Iterator<Item = Linked<Run>> {
        self.tables
            .runs()
            .values()
            .map(move |run| self.clone().link(run))
    }

    /// Finds a Linked<Run> by id.
    pub fn run_by_id(self: Rc<Self>, id: Id64) -> Option<Linked<Run>> {
        self.tables.runs().get(&id).map(|run| self.link(run))
    }

    /// Returns a Vec of Linked<Run> for a given game ID, sorted by category,
    /// level, and then primary time (ascending).
    pub fn runs_by_game_id(self: Rc<Self>, game_id: Id64) -> Option<Vec<Linked<Run>>> {
        self.runs_by_game_id
            .get(&game_id)
            .map(|ref runs| runs.iter().map(|run| self.clone().link(*run)).collect())
    }

    /// Iterator over all Linked<User>s.
    pub fn users(self: Rc<Self>) -> impl Iterator<Item = Linked<User>> {
        self.tables
            .users()
            .values()
            .map(move |user| self.clone().link(user))
    }

    /// Finds a Linked<Run> by id.
    pub fn user_by_id(self: Rc<Self>, id: Id64) -> Option<Linked<User>> {
        self.tables.users().get(&id).map(|user| self.link(user))
    }

    /// Finds a Linked<User> by name.
    pub fn user_by_name(self: Rc<Self>, name: &str) -> Option<Linked<User>> {
        self.users_by_name
            .get(name)
            .map(|user| self.clone().link(*user))
    }

    /// Iterator over all Linked<Game>s.
    pub fn games(self: Rc<Self>) -> impl Iterator<Item = Linked<Game>> {
        self.tables
            .games()
            .values()
            .map(move |game| self.clone().link(game))
    }

    /// Finds a Game<Run> by id.
    pub fn game_by_id(self: Rc<Self>, id: Id64) -> Option<Linked<Game>> {
        self.tables.games().get(&id).map(|game| self.link(game))
    }

    /// Finds a Linked<Game> by slug.
    pub fn game_by_slug(self: Rc<Self>, slug: &str) -> Option<Linked<Game>> {
        self.games_by_slug
            .get(slug)
            .map(|game| self.clone().link(*game))
    }

    /// Finds a level with the given name and game ID.
    pub fn level_by_game_id_and_name(
        self: Rc<Self>,
        game_id: Id64,
        name: &str,
    ) -> Option<Linked<Level>> {
        self.levels_by_game_id_and_name
            .get(&(game_id, name))
            .map(|level| self.clone().link(*level))
    }

    /// Iterator over all Linked<Level>s.
    pub fn levels(self: Rc<Self>) -> impl Iterator<Item = Linked<Level>> {
        self.tables
            .levels()
            .values()
            .map(move |level| self.clone().link(level))
    }

    /// Finds a Level<Run> by id.
    pub fn level_by_id(self: Rc<Self>, id: Id64) -> Option<Linked<Level>> {
        self.tables.levels().get(&id).map(|level| self.link(level))
    }

    /// An iterator over all Linked<Category>s.
    pub fn category_by_id(self: Rc<Self>, id: Id64) -> Option<Linked<Category>> {
        self.tables
            .categories()
            .get(&id)
            .map(|category| self.link(category))
    }

    /// Finds a category with the given name and game ID.
    pub fn category_by_game_id_and_name(
        self: Rc<Self>,
        game_id: Id64,
        name: &str,
    ) -> Option<Linked<Category>> {
        self.categories_by_game_id_and_name
            .get(&(game_id, name))
            .map(|category| self.clone().link(*category))
    }

    /// Iterator over all Linked<Category>s.
    pub fn categories(self: Rc<Self>) -> impl Iterator<Item = Linked<Category>> {
        self.tables
            .categories()
            .values()
            .map(move |category| self.clone().link(category))
    }
}

/// Wraps [Model] types to add references to the Database, adding new
/// accessor methods.
#[derive(Serialize)]
pub struct Linked<ModelType: 'static + Model> {
    #[serde(skip)]
    database: Rc<Database>,
    #[serde(flatten)]
    item: &'static ModelType,
}

impl<ModelType: Model> Linked<ModelType> {
    pub fn new(database: Rc<Database>, item: &'static ModelType) -> Self {
        Self { database, item }
    }
}

impl<ModelType: Model> Deref for Linked<ModelType> {
    type Target = ModelType;

    fn deref(&self) -> &ModelType {
        &self.item
    }
}

impl Linked<Run> {
    /// Returns the Linked<Game> for this Run.
    pub fn game(&self) -> Linked<Game> {
        self.database
            .clone()
            .game_by_id(*self.game_id())
            .expect(DATABASE_UNEXPECTEDLY_INVALID)
    }

    /// Returns the Linked<Category> for this Run.
    pub fn category(&self) -> Linked<Category> {
        self.database
            .clone()
            .category_by_id(*self.category_id())
            .expect(DATABASE_UNEXPECTEDLY_INVALID)
    }

    /// Returns Some(Linked<Level>) for this Run, or None if it's a full-game run.
    pub fn level(&self) -> Option<Linked<Level>> {
        self.level_id().map(|level_id| {
            self.database
                .clone()
                .level_by_id(level_id)
                .expect(DATABASE_UNEXPECTEDLY_INVALID)
        })
    }

    /// Returns Vec<Linked<User>> for this Run. May be empty if all runners
    /// unregistered/guests.
    pub fn users(&self) -> Vec<Linked<User>> {
        self.players()
            .iter()
            .flat_map(|player| match player {
                RunPlayer::UserId(user_id) => Some(
                    self.database
                        .clone()
                        .user_by_id(*user_id)
                        .expect(DATABASE_UNEXPECTEDLY_INVALID),
                ),
                RunPlayer::GuestName(_) => None,
            })
            .collect()
    }

    fn validate(&self) -> Result<(), IntegrityErrors> {
        let mut errors = Vec::new();

        if let None = self.database.clone().game_by_id(*self.game_id()) {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type:       "game",
                target_id:         *self.game_id(),
                source_type:       "run",
                source_id:         *self.id(),
                foreign_key_field: "game_id",
                source:            Box::new(self.item),
            });
        }

        if let None = self.database.clone().category_by_id(*self.category_id()) {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type:       "category",
                target_id:         *self.category_id(),
                source_type:       "run",
                source_id:         *self.id(),
                foreign_key_field: "category_id",
                source:            Box::new(self.item),
            });
        }

        if let Some(level_id) = self.level_id() {
            if let None = self.database.clone().level_by_id(*level_id) {
                errors.push(IntegrityError::ForeignKeyMissing {
                    target_type:       "level",
                    target_id:         *level_id,
                    source_type:       "run",
                    source_id:         *self.id(),
                    foreign_key_field: "level_id",
                    source:            Box::new(self.item),
                });
            }
        }

        for player in self.players() {
            if let RunPlayer::UserId(user_id) = player {
                if let None = self.database.clone().user_by_id(*user_id) {
                    errors.push(IntegrityError::ForeignKeyMissing {
                        target_type:       "user",
                        target_id:         *user_id,
                        source_type:       "run",
                        source_id:         *self.id(),
                        foreign_key_field: "players[…].0",
                        source:            Box::new(self.item),
                    });
                }
            }
        }

        if let Err(validation_errors) = self.item.validate() {
            errors.push(IntegrityError::CheckFailed {
                errors: validation_errors,
                item:   Box::new(self.item),
            });
        }

        IntegrityErrors::try_from(errors)
    }
}

impl Linked<User> {
    fn validate(&self) -> Result<(), IntegrityErrors> {
        let mut errors = Vec::new();

        if let Err(validation_errors) = self.item.validate() {
            errors.push(IntegrityError::CheckFailed {
                errors: validation_errors,
                item:   Box::new(self.item),
            });
        }

        Ok(())
    }
}

impl Linked<Game> {
    /// Returns a Vec of all the verified Runs for this Game.
    pub fn runs(&self) -> Vec<Linked<Run>> {
        self.database
            .clone()
            .runs_by_game_id(*self.id())
            .expect(DATABASE_UNEXPECTEDLY_INVALID)
    }

    pub fn category_by_name(&self, name: &str) -> Option<Linked<Category>> {
        self.database
            .clone()
            .category_by_game_id_and_name(*self.id(), name)
    }

    pub fn level_by_name(&self, name: &str) -> Option<Linked<Level>> {
        self.database
            .clone()
            .level_by_game_id_and_name(*self.id(), name)
    }

    fn validate(&self) -> Result<(), IntegrityErrors> {
        let mut errors = Vec::new();

        if let Err(validation_errors) = self.item.validate() {
            errors.push(IntegrityError::CheckFailed {
                errors: validation_errors,
                item:   Box::new(self.item),
            });
        }

        IntegrityErrors::try_from(errors)
    }
}

impl Linked<Level> {
    /// Returns the Linked<Game> for this Level.
    pub fn game(&self) -> Linked<Game> {
        self.database
            .clone()
            .game_by_id(*self.game_id())
            .expect(DATABASE_UNEXPECTEDLY_INVALID)
    }

    fn validate(&self) -> Result<(), IntegrityErrors> {
        let mut errors = Vec::new();

        if let Err(validation_errors) = self.item.validate() {
            errors.push(IntegrityError::CheckFailed {
                errors: validation_errors,
                item:   Box::new(self.item),
            });
        }

        if let None = self.database.clone().game_by_id(*self.game_id()) {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type:       "game",
                target_id:         *self.game_id(),
                source_type:       "level",
                source_id:         *self.id(),
                foreign_key_field: "game_id",
                source:            Box::new(self.item),
            });
        }

        IntegrityErrors::try_from(errors)
    }
}

impl Linked<Category> {
    /// Returns the Linked<Game> for this Category.
    pub fn game(&self) -> Linked<Game> {
        self.database
            .clone()
            .game_by_id(*self.game_id())
            .expect(DATABASE_UNEXPECTEDLY_INVALID)
    }

    fn validate(&self) -> Result<(), IntegrityErrors> {
        let mut errors = Vec::new();

        if let Err(validation_errors) = self.item.validate() {
            errors.push(IntegrityError::CheckFailed {
                errors: validation_errors,
                item:   Box::new(self.item),
            });
        }

        if let None = self.database.clone().game_by_id(*self.game_id()) {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type:       "game",
                target_id:         *self.game_id(),
                source_type:       "category",
                source_id:         *self.id(),
                foreign_key_field: "game_id",
                source:            Box::new(self.item),
            });
        }

        IntegrityErrors::try_from(errors)
    }
}
