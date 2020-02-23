use std::collections::HashSet;
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
use itertools::Itertools;
#[allow(unused)]
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::{
    data::{models::*, types::*},
    utils::slugify,
};

/// Validates a Database for business and integrity requirements.
pub fn validate(_database: &super::Database) -> Result<(), IntegrityErrors> {
    
        let mut errors = vec![];

        trace!("Validating {} runs.", self.tables.runs().len());
        for run in self.runs() {
            if let Err(mut error) = run.validate() {
                errors.append(&mut error.errors);
            }
        }

        trace!("Validating {} users.", self.tables.users().len());
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

        trace!("Validating {} games.", self.tables.games().len());
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

        trace!("Validating {} categories.", self.tables.categories().len());
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

        trace!("Validating {} levels.", self.tables.levels().len());
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
        pub fn validate(self: &Arc<Self>) -> Result<(), IntegrityErrors> {
            let mut errors = vec![];
    
            trace!("Validating {} runs.", self.tables.runs().len());
            for run in self.runs() {
                if let Err(mut error) = run.validate() {
                    errors.append(&mut error.errors);
                }
            }
    
            trace!("Validating {} users.", self.tables.users().len());
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
    
            trace!("Validating {} games.", self.tables.games().len());
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
    
            trace!("Validating {} categories.", self.tables.categories().len());
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
    
            trace!("Validating {} levels.", self.tables.levels().len());
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

#[derive(Debug, Error, From)]
pub struct IntegrityErrors {
    pub errors: Vec<IntegrityError>,
}

impl IntegrityErrors {
    // TODO: rename this!
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
pub struct RowRefs<'tables> {
    pub games: HashSet<&'tables Game>,
    pub categories: HashSet<&'tables Category>,
    pub levels: HashSet<&'tables Level>,
    pub runs: HashSet<&'tables Run>,
    pub users: HashSet<&'tables User>,
}

impl IntegrityError {
    pub fn invalid_rows<'tables>(&self) -> RowRefs<'tables> {
        let mut refs = RowRefs::default();

        match self {
            IntegrityError::IndexingError => {
                error!("indexing failed");
            }
            IntegrityError::ForeignKeyMissing { source, .. } => {
                use AnyModel::*;
                match source {
                    Game(game) => refs.games.insert(game),
                    Category(category) => refs.categories.insert(category),
                    Level(level) => refs.levels.insert(level),
                    Run(run) => refs.runs.insert(run),
                    User(user) => refs.users.insert(user),
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
                            refs.categories.insert(dupe);
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
                            refs.levels.insert(dupe);
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
                            refs.games.insert(dupe);
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
                            refs.users.insert(dupe);
                        }
                    }
                };
            }
            IntegrityError::MissingPrimaryTiming(run) => {
                refs.runs.insert(run);
            }
        }

        refs
    }
}
