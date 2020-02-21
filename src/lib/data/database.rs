//! The world's worst in-memory database of normalized speedrun data.
use std::{
    collections::{BTreeMap, HashMap},
    default::Default,
    fmt::{Debug, Display},
    ops::Deref,
    sync::Arc,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use err_derive::Error;
use getset::Getters;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::{
    data::{models::*, types::*},
    utils::slugify,
};

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
            if i >= 16 {
                writeln!(f, "     ...and more!")?;
                break
            }

            writeln!(f, "{:4}. {}", i + 1, error)?;
        }
        Ok(())
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error, From)]
pub enum IntegrityError {
    #[error(display = "integrity failure during indexing")]
    IndexingError,
    #[error(
        display = "{} with id {} does not exist, specified by {} in {:#?}",
        target_type,
        target_id,
        foreign_key_field,
        source
    )]
    ForeignKeyMissing {
        target_type:       &'static str,
        target_id:         u64,
        foreign_key_field: &'static str,
        source:            AnyModel,
    },
    #[error(display = "row validation check failed: {:?} in {:?}", errors, source)]
    CheckFailed {
        errors: ValidationErrors,
        source: AnyModel,
    },
    #[error(display = "duplicate {:?} slug for {:?}", slug, sources)]
    NonUniqueSlug {
        slug:    String,
        sources: AnyModelVec,
    },
    #[error(display = "run is missing primary timing: {:?}", _0)]
    MissingPrimaryTiming(Run),
}

/// All of the speedrun data in our normalized format.bash, indexed by ID.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Getters)]
#[get = "pub"]
pub struct Tables {
    runs:       BTreeMap<u64, Run>,
    users:      BTreeMap<u64, User>,
    games:      BTreeMap<u64, Game>,
    categories: BTreeMap<u64, Category>,
    levels:     BTreeMap<u64, Level>,
}

#[derive(Debug, Clone)]
pub struct Indicies<'tables> {
    last_updated:                                 DateTime<Utc>,
    runs_by_game_id_and_category_id_and_level_id:
        BTreeMap<(u64, u64, Option<u64>), Vec<&'tables Run>>,
    games_by_slug:                                BTreeMap<String, &'tables Game>,
    users_by_slug:                                BTreeMap<String, &'tables User>,
    per_game_categories_by_game_id_and_slug: BTreeMap<(u64, String), &'tables Category>,
    per_level_categories_by_game_id_and_slug: BTreeMap<(u64, String), &'tables Category>,
    levels_by_game_id_and_slug:                   BTreeMap<(u64, String), &'tables Level>,
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
use rentals::*;

// Dereference Database -> Indicies -> Tables

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
/// possible, because it must have already been validated, such as for foreign
/// key lookups.
const DATABASE_INTEGRITY: &str = "Database state invalid despite passing validation?!";

impl Database {
    pub fn link<ModelType: Model>(
        self: &Arc<Self>,
        item: &'static ModelType,
    ) -> Linked<ModelType> {
        Linked::new(self.clone(), item)
    }

    /// Creates a new Database indexing a collection of static tables.
    pub fn from_tables(tables: Arc<Tables>) -> Result<Arc<Self>, IntegrityErrors> {
        let maybe_self: Result<Self, _> = Self::try_new(tables, |tables| {
            let mut last_updated: DateTime<Utc> =
                DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);
            let mut runs_by_game_id_and_category_id_and_level_id: BTreeMap<
                (u64, u64, Option<u64>),
                Vec<&'_ Run>,
            > = Default::default();
            let mut games_by_slug: BTreeMap<String, &'_ Game> = Default::default();
            let mut users_by_slug: BTreeMap<String, &'_ User> = Default::default();
            let mut per_game_categories_by_game_id_and_slug: BTreeMap<
                (u64, String),
                &'_ Category,
            > = Default::default();
            let mut per_level_categories_by_game_id_and_slug: BTreeMap<
                (u64, String),
                &'_ Category,
            > = Default::default();
            let mut levels_by_game_id_and_slug: BTreeMap<(u64, String), &'_ Level> =
                Default::default();
            let index_errored = '_indexing: {
                for game in tables.games().values() {
                    games_by_slug.insert(game.slug().to_string(), game);
                }

                for run in tables.runs().values() {
                    if let Some(created) = run.created {
                        if created > last_updated {
                            last_updated = created;
                        }
                    }

                    let key = (*run.game_id(), *run.category_id(), *run.level_id());
                    if let Some(runs) =
                        runs_by_game_id_and_category_id_and_level_id.get_mut(&key)
                    {
                        runs.push(run);
                    } else {
                        runs_by_game_id_and_category_id_and_level_id
                            .insert(key, vec![run])
                            .unwrap_none();
                    }
                }

                for user in tables.users().values() {
                    users_by_slug.insert(user.slug().to_string(), user);
                }

                for category in tables.categories().values() {
                    match category.per() {
                        CategoryType::PerGame =>
                            &mut per_game_categories_by_game_id_and_slug,
                        CategoryType::PerLevel =>
                            &mut per_level_categories_by_game_id_and_slug,
                    }
                    .insert((*category.game_id(), category.slug().to_string()), category);
                }

                for level in tables.levels().values() {
                    levels_by_game_id_and_slug
                        .insert((*level.game_id(), level.slug().to_string()), level);
                }

                for game_runs in runs_by_game_id_and_category_id_and_level_id.values_mut() {
                    game_runs.sort();
                }

                false
            };

            let mut errors = Vec::new();
            if index_errored {
                error!("indexing failed, database must have validity errors");
                errors.push(IntegrityError::IndexingError)
            }

            IntegrityErrors::try_from(errors).map(|_| Indicies {
                last_updated,
                runs_by_game_id_and_category_id_and_level_id,
                games_by_slug,
                users_by_slug,
                per_game_categories_by_game_id_and_slug,
                per_level_categories_by_game_id_and_slug,
                levels_by_game_id_and_slug,
            })
        });

        let mut errors = Vec::new();
        let self_: Result<Arc<Self>, IntegrityErrors> = match maybe_self {
            Ok(self_) => Ok(Arc::new(self_)),
            Err(error) => unimplemented!("TODO: copy these errors"),
        };

        let self_ = self_.unwrap();

        if let Err(mut errors_) = self_.validate() {
            errors.append(&mut errors_.errors);
        }

        IntegrityErrors::try_from(errors).map(|_| self_)
    }

    pub fn tables(&self) -> &Tables {
        self.all().tables
    }

    pub fn indicies(&self) -> &Indicies {
        self.all().indicies
    }

    pub fn validate(self: &Arc<Self>) -> Result<(), IntegrityErrors> {
        let mut errors = vec![];

        trace!("Validating {} runs.", self.tables().runs().len());
        for run in self.runs() {
            if let Err(mut error) = run.validate() {
                errors.append(&mut error.errors);
            }
        }

        trace!("Validating {} users.", self.tables().users().len());
        let mut user_slugs = HashMap::<String, Vec<User>>::new();
        for user in self.users() {
            if let Err(mut error) = user.validate() {
                errors.append(&mut error.errors);
            } else {
                user_slugs
                    .entry(user.slug().to_string())
                    .or_insert_with(Vec::new)
                    .push(User::clone(&*user));
            }
        }
        for (slug, items) in user_slugs {
            if items.len() >= 2 {
                errors.push(IntegrityError::NonUniqueSlug {
                    slug,
                    sources: AnyModelVec::Users(items),
                });
            }
        }

        trace!("Validating {} games.", self.tables().games().len());
        let mut game_slugs = HashMap::<String, Vec<Game>>::new();
        for game in self.games() {
            if let Err(mut error) = game.validate() {
                errors.append(&mut error.errors);
            } else {
                game_slugs
                    .entry(game.slug().to_string())
                    .or_insert_with(Vec::new)
                    .push(Game::clone(&*game));
            }
        }
        for (slug, items) in game_slugs {
            if items.len() >= 2 {
                errors.push(IntegrityError::NonUniqueSlug {
                    slug,
                    sources: AnyModelVec::Games(items),
                });
            }
        }

        trace!(
            "Validating {} categories.",
            self.tables().categories().len()
        );
        let mut category_slugs = HashMap::<String, Vec<Category>>::new();
        for category in self.categories() {
            if let Err(mut error) = category.validate() {
                errors.append(&mut error.errors);
            } else {
                let slug = format!(
                    "{}/{}/{}",
                    category.game().slug(),
                    slugify(&format!("{:?}", category.per())),
                    category.slug()
                );
                category_slugs
                    .entry(slug)
                    .or_insert_with(Vec::new)
                    .push(Category::clone(&*category));
            }
        }
        for (slug, items) in category_slugs {
            if items.len() >= 2 {
                errors.push(IntegrityError::NonUniqueSlug {
                    slug,
                    sources: AnyModelVec::Categories(items),
                });
            }
        }

        trace!("Validating {} levels.", self.tables().levels().len());
        let mut level_slugs = HashMap::<String, Vec<Level>>::new();
        for level in self.levels() {
            if let Err(mut error) = level.validate() {
                errors.append(&mut error.errors);
            } else {
                let slug = format!("{}/{}", level.game().slug(), level.slug());
                level_slugs
                    .entry(slug)
                    .or_insert_with(Vec::new)
                    .push(Level::clone(&*level));
            }
        }
        for (slug, items) in level_slugs {
            if items.len() >= 2 {
                errors.push(IntegrityError::NonUniqueSlug {
                    slug,
                    sources: AnyModelVec::Levels(items),
                });
            }
        }

        IntegrityErrors::try_from(errors)
    }

    /// Iterator over all Linked<Run>s.
    pub fn runs(self: &Arc<Self>) -> impl Iterator<Item = Linked<Run>> {
        let self_ = self.clone();
        self.tables()
            .runs()
            .values()
            .map(move |run| self_.link(run))
    }

    /// Finds a Linked<Run> by id.
    pub fn run_by_id(self: &Arc<Self>, id: u64) -> Option<Linked<Run>> {
        self.tables().runs().get(&id).map(|run| self.link(run))
    }

    /// Returns a Vec of Linked<Run> for a given game ID, sorted by category,
    /// level, and then primary time (ascending).
    pub fn runs_by_game_id(self: &Arc<Self>, game_id: u64) -> Vec<Linked<Run>> {
        self.runs_by_game_id_and_category_id_and_level_id
            .range((game_id, 0, None)..(game_id + 1, 0, None))
            .map(|(_key, value)| value)
            .flat_map(|ref runs| runs.iter().map(|run| self.link(*run)).collect::<Vec<_>>())
            .collect()
    }

    /// Iterator over all Linked<User>s.
    pub fn users(self: &Arc<Self>) -> impl Iterator<Item = Linked<User>> {
        let self_ = self.clone();
        self.tables
            .users()
            .values()
            .map(move |user| self_.link(user))
    }

    /// Finds a Linked<Run> by id.
    pub fn user_by_id(self: &Arc<Self>, id: u64) -> Option<Linked<User>> {
        self.tables().users().get(&id).map(|user| self.link(user))
    }

    /// Finds a Linked<User> by name.
    pub fn user_by_slug(self: &Arc<Self>, slug: &str) -> Option<Linked<User>> {
        // TODO: stop all indexing by slugify, let consumers do that if they want.
        self.users_by_slug.get(slug).map(|user| self.link(*user))
    }

    /// Iterator over all Linked<Game>s.
    pub fn games(self: &Arc<Self>) -> impl Iterator<Item = Linked<Game>> {
        let self_ = self.clone();
        self.tables
            .games()
            .values()
            .map(move |game| self_.link(game))
    }

    /// Finds a Game<Run> by id.
    pub fn game_by_id(self: &Arc<Self>, id: u64) -> Option<Linked<Game>> {
        self.tables().games().get(&id).map(|game| self.link(game))
    }

    /// Finds a Linked<Game> by slug.
    pub fn game_by_slug(self: &Arc<Self>, slug: &str) -> Option<Linked<Game>> {
        self.games_by_slug.get(slug).map(|game| self.link(*game))
    }

    pub fn levels_by_game_id(self: &Arc<Self>, game_id: u64) -> Vec<Linked<Level>> {
        self.levels_by_game_id_and_slug
            .range((game_id, "".to_string())..(game_id + 1, "".to_string()))
            .map(|(_key, level)| self.link(*level))
            .collect()
    }

    /// Finds a level with the given name and game ID.
    pub fn level_by_game_id_and_slug(
        self: &Arc<Self>,
        game_id: u64,
        slug: &str,
    ) -> Option<Linked<Level>> {
        self.levels_by_game_id_and_slug
            .get(&(game_id, slug.to_string()))
            .map(|level| self.link(*level))
    }

    /// Iterator over all Linked<Level>s.
    pub fn levels(self: &Arc<Self>) -> impl Iterator<Item = Linked<Level>> {
        let self_ = self.clone();
        self.tables
            .levels()
            .values()
            .map(move |level| self_.link(level))
    }

    /// Finds a Level<Run> by id.
    pub fn level_by_id(self: &Arc<Self>, id: u64) -> Option<Linked<Level>> {
        self.tables()
            .levels()
            .get(&id)
            .map(|level| self.link(level))
    }

    /// An iterator over all Linked<Category>s.
    pub fn category_by_id(self: &Arc<Self>, id: u64) -> Option<Linked<Category>> {
        self.tables
            .categories()
            .get(&id)
            .map(|category| self.link(category))
    }

    /// Finds a per-game category with the given slug and game ID.
    pub fn per_game_category_by_game_id_and_slug(
        self: &Arc<Self>,
        game_id: u64,
        slug: &str,
    ) -> Option<Linked<Category>> {
        self.per_game_categories_by_game_id_and_slug
            .get(&(game_id, slug.to_string()))
            .map(|category| self.link(*category))
    }

    /// Finds a per-level category with the given slug and game ID.
    pub fn per_level_category_by_game_id_and_slug(
        self: &Arc<Self>,
        game_id: u64,
        slug: &str,
    ) -> Option<Linked<Category>> {
        self.per_level_categories_by_game_id_and_slug
            .get(&(game_id, slug.to_string()))
            .map(|category| self.link(*category))
    }

    /// Iterator over all Linked<Category>s.
    pub fn categories(self: &Arc<Self>) -> impl Iterator<Item = Linked<Category>> {
        let self_ = self.clone();
        self.tables
            .categories()
            .values()
            .map(move |category| self_.link(category))
    }
}

/// Wraps [Model] types to add references to the Database, adding new
/// accessor methods.
#[derive(Serialize, Debug, Clone)]
pub struct Linked<ModelType: 'static + Model + Debug> {
    #[serde(skip)]
    database: Arc<Database>,
    #[serde(flatten)]
    item:     &'static ModelType,
}

impl<ModelType: Model> Linked<ModelType> {
    pub fn new(database: Arc<Database>, item: &'static ModelType) -> Self {
        Self { database, item }
    }

    /// Returns the underlying static model instance reference.
    pub fn as_static(&self) -> &'static ModelType {
        self.item
    }
}

impl<ModelType: Model> Deref for Linked<ModelType> {
    type Target = ModelType;

    /// Deref to enable method delegation, but this loses the 'static lifetime.
    fn deref(&self) -> &ModelType {
        &self.item
    }
}

impl Linked<Run> {
    /// Returns the primary timing for this run.
    pub fn time_ms(&self) -> u64 {
        self.times_ms()
            .get(self.game().primary_timing())
            .expect(DATABASE_INTEGRITY)
    }

    /// Returns the Linked<Game> for this Run.
    pub fn game(&self) -> Linked<Game> {
        self.database
            .clone()
            .game_by_id(*self.game_id())
            .expect(DATABASE_INTEGRITY)
    }

    /// Returns the Linked<Category> for this Run.
    pub fn category(&self) -> Linked<Category> {
        self.database
            .clone()
            .category_by_id(*self.category_id())
            .expect(DATABASE_INTEGRITY)
    }

    /// Returns Some(Linked<Level>) for this Run, or None if it's a full-game run.
    pub fn level(&self) -> Option<Linked<Level>> {
        self.level_id().map(|level_id| {
            self.database
                .clone()
                .level_by_id(level_id)
                .expect(DATABASE_INTEGRITY)
        })
    }

    /// Returns a URL-safe slug identifying this run within its given game,
    /// category, and level (it may conflict with slugs in others).
    pub fn slug(&self) -> String {
        let users = self.users();
        let src_id = self.src_id();
        if users.is_empty() {
            src_id
        } else {
            format!(
                "{}-{}",
                users
                    .iter()
                    .map(|u| u.slug())
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("-"),
                &src_id[..4]
            )
        }
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
                        .expect(DATABASE_INTEGRITY),
                ),
                RunPlayer::GuestName(_name) => None,
            })
            .collect()
    }

    fn validate(&self) -> Result<(), IntegrityErrors> {
        let mut errors = Vec::new();

        if self.database.game_by_id(*self.game_id()).is_none() {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type:       "game",
                target_id:         *self.game_id(),
                foreign_key_field: "game_id",
                source:            (*self.item).clone().into(),
            });
        } else {
            let game = self.game();
            let primary_timing = game.primary_timing();
            let times = self.times_ms();
            if times.get(primary_timing).is_none() {
                errors.push(IntegrityError::MissingPrimaryTiming((**self).clone()))
            }
        }

        if self
            .database
            .clone()
            .category_by_id(*self.category_id())
            .is_none()
        {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type:       "category",
                target_id:         *self.category_id(),
                foreign_key_field: "category_id",
                source:            (*self.item).clone().into(),
            });
        }

        if let Some(level_id) = self.level_id() {
            if self.database.level_by_id(*level_id).is_none() {
                errors.push(IntegrityError::ForeignKeyMissing {
                    target_type:       "level",
                    target_id:         *level_id,
                    foreign_key_field: "level_id",
                    source:            (*self.item).clone().into(),
                });
            }
        }

        for player in self.players() {
            if let RunPlayer::UserId(user_id) = player {
                if self.database.user_by_id(*user_id).is_none() {
                    errors.push(IntegrityError::ForeignKeyMissing {
                        target_type:       "user",
                        target_id:         *user_id,
                        foreign_key_field: "players[…].0",
                        source:            (*self.item).clone().into(),
                    });
                }
            }
        }

        if let Err(validation_errors) = self.item.validate() {
            errors.push(IntegrityError::CheckFailed {
                errors: validation_errors,
                source: (*self.item).clone().into(),
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
                source: (*self.item).clone().into(),
            });
        }

        Ok(())
    }
}

impl Linked<Game> {
    /// Returns a Vec of all the verified Runs for this Game.
    pub fn runs(&self) -> Vec<Linked<Run>> {
        self.database.clone().runs_by_game_id(*self.id())
    }

    pub fn per_game_category_by_slug(&self, slug: &str) -> Option<Linked<Category>> {
        self.database
            .clone()
            .per_game_category_by_game_id_and_slug(*self.id(), slug)
    }

    pub fn per_level_category_by_slug(&self, slug: &str) -> Option<Linked<Category>> {
        self.database
            .clone()
            .per_level_category_by_game_id_and_slug(*self.id(), slug)
    }

    pub fn level_by_slug(&self, slug: &str) -> Option<Linked<Level>> {
        self.database
            .clone()
            .level_by_game_id_and_slug(*self.id(), slug)
    }

    fn validate(&self) -> Result<(), IntegrityErrors> {
        let mut errors = Vec::new();

        if let Err(validation_errors) = self.item.validate() {
            errors.push(IntegrityError::CheckFailed {
                errors: validation_errors,
                source: (*self.item).clone().into(),
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
            .expect(DATABASE_INTEGRITY)
    }

    fn validate(&self) -> Result<(), IntegrityErrors> {
        let mut errors = Vec::new();

        if let Err(validation_errors) = self.item.validate() {
            errors.push(IntegrityError::CheckFailed {
                errors: validation_errors,
                source: (*self.item).clone().into(),
            });
        }

        if self.database.game_by_id(*self.game_id()).is_none() {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type:       "game",
                target_id:         *self.game_id(),
                foreign_key_field: "game_id",
                source:            (*self.item).clone().into(),
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
            .expect(DATABASE_INTEGRITY)
    }

    pub fn runs(&self) -> Vec<Linked<Run>> {
        self.database
            .clone()
            .runs_by_game_id(*self.game_id())
            .iter()
            .filter(|r| r.category_id() == self.id())
            .cloned()
            .collect()
    }

    pub fn full_runs(&self) -> Vec<Linked<Run>> {
        let mut runs = self.runs();
        runs.retain(|run| run.level_id().is_none());
        runs
    }

    pub fn level_runs(&self, level: &Level) -> Vec<Linked<Run>> {
        let mut runs = self.runs();
        runs.retain(|run| *run.level_id() == Some(*level.id()));
        runs
    }

    fn validate(&self) -> Result<(), IntegrityErrors> {
        let mut errors = Vec::new();

        if let Err(validation_errors) = self.item.validate() {
            errors.push(IntegrityError::CheckFailed {
                errors: validation_errors,
                source: (*self.item).clone().into(),
            });
        }

        if self.database.game_by_id(*self.game_id()).is_none() {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type:       "game",
                target_id:         *self.game_id(),
                foreign_key_field: "game_id",
                source:            (*self.item).clone().into(),
            });
        }

        IntegrityErrors::try_from(errors)
    }
}
