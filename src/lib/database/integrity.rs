use std::collections::HashSet;
use std::{
    collections::HashMap,
    default::Default,
    fmt::{Debug, Display},
};

use derive_more::From;
use err_derive::Error;
use itertools::Itertools;

use log::{debug, error, info, trace, warn};
use validator::{Validate, ValidationErrors};

use speedruns_models::{
    any::{AnyModel, AnyModelVec},
    Category, Game, Level, Run, RunPlayer, User,
};
use speedruns_utils::slugify;

// We're using the validator::Validator trait in our data model, but
// TODO: we probably want to stop doing that. Here we add further validation
// and integrity checking that requires the context of the Database.

/// Validates a Database for business and integrity requirements.
pub fn validate(database: &super::Database) -> Result<(), IntegrityErrors> {
    let mut errors = vec![];

    trace!("Validating {} runs.", database.runs().len());
    for run in database.runs().values() {
        if let Err(mut error) = validate_run(database, &run) {
            errors.append(&mut error.errors);
        }
    }

    trace!("Validating {} users.", database.users().len());
    let mut user_slugs = HashMap::<String, Vec<User>>::new();
    for user in database.users().values() {
        if let Err(mut error) = validate_user(database, &user) {
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

    trace!("Validating {} games.", database.games().len());
    let mut game_slugs = HashMap::<String, Vec<Game>>::new();
    for game in database.games().values() {
        if let Err(mut error) = validate_game(database, &game) {
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

    trace!("Validating {} categories.", database.categories().len());
    let mut category_slugs = HashMap::<String, Vec<Category>>::new();
    for category in database.categories().values() {
        if let Err(mut error) = validate_category(database, &category) {
            errors.append(&mut error.errors);
        } else {
            let game = &database.games()[&category.game_id];
            let slug = format!(
                "{}/{}/{}",
                game.slug(),
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

    trace!("Validating {} levels.", database.levels().len());
    let mut level_slugs = HashMap::<String, Vec<Level>>::new();
    for level in database.levels().values() {
        if let Err(mut error) = validate_level(database, &level) {
            errors.append(&mut error.errors);
        } else {
            let game = &database.games()[&level.game_id];
            let slug = format!("{}/{}", game.slug(), level.slug());
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

fn validate_game(database: &super::Database, game: &Game) -> Result<(), IntegrityErrors> {
    let mut errors = Vec::new();

    if let Err(validation_errors) = game.validate() {
        errors.push(IntegrityError::CheckFailed {
            errors: validation_errors,
            source: game.clone().into(),
        });
    }

    IntegrityErrors::try_from(errors)
}

fn validate_category(
    database: &super::Database,
    category: &Category,
) -> Result<(), IntegrityErrors> {
    let mut errors = Vec::new();

    if let Err(validation_errors) = category.validate() {
        errors.push(IntegrityError::CheckFailed {
            errors: validation_errors,
            source: category.clone().into(),
        });
    }

    if database.games().get(&category.game_id).is_none() {
        errors.push(IntegrityError::ForeignKeyMissing {
            target_type: "game",
            target_id: category.game_id,
            foreign_key_field: "game_id",
            source: category.clone().into(),
        });
    }

    IntegrityErrors::try_from(errors)
}

fn validate_level(
    database: &super::Database,
    level: &Level,
) -> Result<(), IntegrityErrors> {
    let mut errors = Vec::new();

    if let Err(validation_errors) = level.validate() {
        errors.push(IntegrityError::CheckFailed {
            errors: validation_errors,
            source: level.clone().into(),
        });
    }

    if database.games().get(&level.game_id).is_none() {
        errors.push(IntegrityError::ForeignKeyMissing {
            target_type: "game",
            target_id: level.game_id,
            foreign_key_field: "game_id",
            source: level.clone().into(),
        });
    }

    IntegrityErrors::try_from(errors)
}

fn validate_run(database: &super::Database, run: &Run) -> Result<(), IntegrityErrors> {
    let mut errors = Vec::new();

    match database.games().get(&run.game_id) {
        Some(game) => {
            let primary_timing = game.primary_timing();
            let times = run.times_ms();
            if times.get(primary_timing).is_none() {
                errors.push(IntegrityError::MissingPrimaryTiming(run.clone()))
            }
        }
        None => {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type: "game",
                target_id: run.game_id,
                foreign_key_field: "game_id",
                source: run.clone().into(),
            });
        }
    }

    if database.categories().get(&run.category_id()).is_none() {
        errors.push(IntegrityError::ForeignKeyMissing {
            target_type: "category",
            target_id: run.category_id,
            foreign_key_field: "category_id",
            source: run.clone().into(),
        });
    }

    if let Some(level_id) = run.level_id {
        if database.levels().get(&level_id).is_none() {
            errors.push(IntegrityError::ForeignKeyMissing {
                target_type: "level",
                target_id: level_id,
                foreign_key_field: "level_id",
                source: run.clone().into(),
            });
        }
    }

    for player in run.players() {
        if let RunPlayer::UserId(user_id) = player {
            if database.users().get(&user_id).is_none() {
                errors.push(IntegrityError::ForeignKeyMissing {
                    target_type: "user",
                    target_id: *user_id,
                    foreign_key_field: "players[…].0",
                    source: run.clone().into(),
                });
            }
        }
    }

    if let Err(validation_errors) = run.validate() {
        errors.push(IntegrityError::CheckFailed {
            errors: validation_errors,
            source: run.clone().into(),
        });
    }

    IntegrityErrors::try_from(errors)
}

fn validate_user(database: &super::Database, user: &User) -> Result<(), IntegrityErrors> {
    let mut errors = Vec::new();

    if let Err(validation_errors) = user.validate() {
        errors.push(IntegrityError::CheckFailed {
            errors: validation_errors,
            source: user.clone().into(),
        });
    }

    Ok(())
}

impl Display for IntegrityErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{} IntegrityErrors:", self.errors.len())?;
        for (i, error) in self.errors.iter().enumerate() {
            if i >= 16 {
                writeln!(f, "     ...and more!")?;
                break;
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
        target_type: &'static str,
        target_id: u64,
        foreign_key_field: &'static str,
        source: AnyModel,
    },
    #[error(display = "row validation check failed: {:?} in {:?}", errors, source)]
    CheckFailed {
        errors: ValidationErrors,
        source: AnyModel,
    },
    #[error(display = "duplicate {:?} slug for {:?}", slug, sources)]
    NonUniqueSlug { slug: String, sources: AnyModelVec },
    #[error(display = "run is missing primary timing: {:?}", _0)]
    MissingPrimaryTiming(Run),
}
#[derive(Debug, Clone, Default)]
pub struct Rows {
    pub games: HashSet<Game>,
    pub categories: HashSet<Category>,
    pub levels: HashSet<Level>,
    pub runs: HashSet<Run>,
    pub users: HashSet<User>,
}

impl IntegrityError {
    pub fn invalid_rows<'tables>(&self) -> Rows {
        let mut invalids = Rows::default();

        match self {
            IntegrityError::IndexingError => {
                error!("indexing failed");
            }
            IntegrityError::ForeignKeyMissing { source, .. } => {
                use AnyModel::*;
                match source {
                    Game(game) => invalids.games.insert(game.clone()),
                    Category(category) => invalids.categories.insert(category.clone()),
                    Level(level) => invalids.levels.insert(level.clone()),
                    Run(run) => invalids.runs.insert(run.clone()),
                    User(user) => invalids.users.insert(user.clone()),
                };
            }
            IntegrityError::CheckFailed { .. } => {
                // validation errors shouldn't be possible, they're a sanity check.
                panic!("validation failure?! import bug?");
            }
            IntegrityError::NonUniqueSlug { sources, .. } => {
                use AnyModelVec::*;
                match sources {
                    Categories(categories) => {
                        let dead_dupes = categories
                            .iter()
                            .sorted_by_key(|category| {
                                (category.name().len(), category.name(), category.id())
                                    .clone()
                            })
                            .skip(1);
                        for dupe in dead_dupes {
                            invalids.categories.insert(dupe.clone());
                        }
                    }
                    Levels(levels) => {
                        let dead_dupes = levels
                            .iter()
                            .sorted_by_key(|level| {
                                (level.name().len(), level.name(), level.id()).clone()
                            })
                            .skip(1);
                        for dupe in dead_dupes {
                            invalids.levels.insert(dupe.clone());
                        }
                    }
                    Runs(_) => unreachable!("runs don't have slugs?!"),
                    Games(games) => {
                        let dead_dupes = games
                            .iter()
                            .sorted_by_key(|game| {
                                (
                                    game.created(),
                                    game.slug().len(),
                                    game.name().len(),
                                    game.name(),
                                    game.id(),
                                )
                                    .clone()
                            })
                            .skip(1);
                        for dupe in dead_dupes {
                            invalids.games.insert(dupe.clone());
                        }
                    }
                    Users(users) => {
                        let dead_dupes = users
                            .iter()
                            .sorted_by_key(|user| {
                                (user.created(), user.name().len(), user.name(), user.id())
                                    .clone()
                            })
                            .skip(1);
                        for dupe in dead_dupes {
                            invalids.users.insert(dupe.clone());
                        }
                    }
                };
            }
            IntegrityError::MissingPrimaryTiming(run) => {
                invalids.runs.insert(run.clone());
            }
        }

        invalids
    }
}

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
