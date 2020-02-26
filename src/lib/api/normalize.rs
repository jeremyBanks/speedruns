use derive_more::From;
use err_derive::Error;
use lazy_static::lazy_static;

use log::{debug, error, info, trace, warn};
use regex::Regex;
use validator::Validate;

use speedruns_models::*;
use speedruns_utils::{self as utils, slugify, u64_from_base36};

#[derive(Debug, Error, From)]
pub enum Error {
    #[error(display = "all names were None or zero-length")]
    NoNames,
    #[error(display = "an ID was invalid and could not be decoded: {:?}", _0)]
    InvalidId(utils::Base36DecodingError),
    #[error(display = "internal error: invalid object created. {:?}", _0)]
    InternalValidationErrors(validator::ValidationErrors),
}

pub trait Normalize {
    type Normalized;
    fn normalize(&self) -> Result<Self::Normalized, Error>;
}

impl Normalize for crate::types::User {
    type Normalized = User;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        let name = self
            .names()
            .normalize()
            .unwrap_or_else(|_| format!("Corrupt User {}", self.id()));
        let slug = slugify(&name);
        let user = User {
            id: u64_from_base36(self.id())?,
            created: *self.signup(),
            name,
            slug,
        };

        user.validate()?;

        Ok(user)
    }
}

impl Normalize for crate::types::Names {
    type Normalized = String;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        if let Some(name) = self.international() {
            if !name.is_empty() {
                return Ok(name.to_string());
            }
        }
        if let Some(name) = self.international() {
            if !name.is_empty() {
                return Ok(name.to_string());
            }
        }
        if let Some(name) = self.japanese() {
            if !name.is_empty() {
                return Ok(name.to_string());
            }
        }
        Err(Error::NoNames)
    }
}

impl Normalize for crate::types::Game {
    type Normalized = (Game, Vec<Category>, Vec<Level>);

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        let game = Game {
            id: u64_from_base36(self.id())?,
            name: self.names().normalize()?,
            slug: self.abbreviation().to_string(),
            created: *self.created(),
            primary_timing: self.ruleset().default_time().normalize()?,
        };
        game.validate()?;

        let categories = self
            .categories()
            .iter()
            .map(|api_category| -> Result<Category, Error> {
                let category = Category {
                    game_id: u64_from_base36(self.id())?,
                    id: u64_from_base36(api_category.id())?,
                    slug: slugify(api_category.name()),
                    name: api_category.name().to_string(),
                    rules: api_category.rules().clone().unwrap_or_else(String::new),
                    per: api_category.type_().normalize()?,
                };

                category.validate()?;

                Ok(category)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let levels = self
            .levels()
            .iter()
            .map(|api_level| -> Result<Level, Error> {
                let level = Level {
                    game_id: u64_from_base36(self.id())?,
                    id: u64_from_base36(api_level.id())?,
                    slug: slugify(api_level.name()),
                    name: api_level.name().to_string(),
                    rules: api_level.rules().clone().unwrap_or_default(),
                };

                level.validate()?;

                Ok(level)
            })
            .collect::<Result<_, _>>()?;

        Ok((game, categories, levels))
    }
}

impl Normalize for crate::types::Run {
    // Option because we drop runs that aren't verified.
    type Normalized = Option<Run>;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        match self.status() {
            crate::types::RunStatus::Verified { .. } => {
                let run = Run {
                    game_id: u64_from_base36(self.game())?,
                    id: u64_from_base36(self.id())?,
                    created: *self.submitted(),
                    date: *self.date(),
                    category_id: u64_from_base36(self.category())?,
                    level_id: match self.level() {
                        None => None,
                        Some(level_id) => Some(u64_from_base36(level_id)?),
                    },
                    times_ms: self.times().normalize()?,
                    players: self
                        .players()
                        .iter()
                        .map(Normalize::normalize)
                        .map(Result::unwrap)
                        .collect(),
                    videos: self
                        .videos()
                        .as_ref()
                        .map(|video| {
                            video
                                .links()
                                .clone()
                                .unwrap_or_default()
                                .iter()
                                .map(|uri| uri.to_string().parse().unwrap())
                                .collect()
                        })
                        .unwrap_or_default(),
                };
                run.validate()?;
                Ok(Some(run))
            }
            _ => Ok(None),
        }
    }
}

impl Normalize for crate::types::RunPlayer {
    type Normalized = RunPlayer;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        Ok(match self {
            crate::types::RunPlayer::Guest { name, .. } => {
                RunPlayer::GuestName(name.to_string())
            }
            crate::types::RunPlayer::User { id, .. } => {
                RunPlayer::UserId(u64_from_base36(id)?)
            }
        })
    }
}

impl Normalize for crate::types::CategoryType {
    type Normalized = CategoryType;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        Ok(match self {
            crate::types::CategoryType::PerLevel => CategoryType::PerLevel,
            crate::types::CategoryType::PerGame => CategoryType::PerGame,
        })
    }
}

impl Normalize for crate::types::GameRulesetTiming {
    type Normalized = TimingMethod;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        Ok(match self {
            crate::types::GameRulesetTiming::IGT => TimingMethod::IGT,
            crate::types::GameRulesetTiming::RTA => TimingMethod::RTA,
            crate::types::GameRulesetTiming::RTA_NL => TimingMethod::RTA_NL,
        })
    }
}

impl Normalize for crate::types::RunTimes {
    type Normalized = RunTimesMs;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        fn u64_or_zero(s: Option<regex::Match<'_>>) -> u64 {
            match s {
                Some(s) => {
                    let s = s.as_str();
                    if s.is_empty() {
                        0
                    } else {
                        s.parse().unwrap()
                    }
                }
                None => 0,
            }
        }

        fn parse_duration_ms(s: &str) -> u64 {
            lazy_static! {
                static ref RE: Regex = Regex::new(
                    r"(?x)
                    P
                    (?:(\d+)D)?
                    T
                    (?:(\d+)H)?
                    (?:(\d+)M)?
                    (?:
                        (\d+)
                        (?:\.(\d\d\d))?
                        S
                    )?
                "
                )
                .unwrap();
            }

            let captures = RE.captures(s).expect("duration regex to cover all cases");
            let days = u64_or_zero(captures.get(1));
            let hours = u64_or_zero(captures.get(2));
            let minutes = u64_or_zero(captures.get(3));
            let seconds = u64_or_zero(captures.get(4));
            let millis = u64_or_zero(captures.get(5));

            ((((days * 24) + hours) * 60 + minutes) * 60 + seconds) * 1000 + millis
        }

        Ok(RunTimesMs {
            igt: self.ingame().as_ref().map(|s| parse_duration_ms(s)),
            rta: self.realtime().as_ref().map(|s| parse_duration_ms(s)),
            rta_nl: self
                .realtime_noloads()
                .as_ref()
                .map(|s| parse_duration_ms(s)),
        })
    }
}
