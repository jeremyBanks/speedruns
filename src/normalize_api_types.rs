use derive_more::From;
use err_derive::Error;

use crate::{api_types as api, normalized_types::*, p64_from_base36};

#[derive(Debug, Error, From)]
pub enum Error {
    #[error(display = "all names were None or zero-length")]
    NoNames,
    #[error(display = "an ID was invalid and could not be decoded: {:?}", _0)]
    InvalidId(crate::utils::Error),
}

pub trait Normalize {
    type Normalized;
    fn normalize(&self) -> Result<Self::Normalized, Error>;
}

impl Normalize for api::User {
    type Normalized = User;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        Ok(User {
            id:      p64_from_base36(self.id())?,
            name:    self.names().normalize()?,
            created: self.signup().clone(),
        })
    }
}

impl Normalize for api::Names {
    type Normalized = String;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        if let Some(name) = self.international() {
            if name.len() > 0 {
                return Ok(name.to_string())
            }
        }
        if let Some(name) = self.international() {
            if name.len() > 0 {
                return Ok(name.to_string())
            }
        }
        if let Some(name) = self.japanese() {
            if name.len() > 0 {
                return Ok(name.to_string())
            }
        }
        Err(Error::NoNames.into())
    }
}

impl Normalize for api::Game {
    type Normalized = (Game, Vec<Category>, Vec<Level>);

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        let game = Game {
            id:      p64_from_base36(self.id())?,
            name:    self.names().normalize()?,
            slug:    self.abbreviation().clone(),
            created: self.created().clone(),
        };
        let categories = vec![];
        let levels = vec![];
        Ok((game, categories, levels))
    }
}

impl Normalize for api::Run {
    type Normalized = Run;

    fn normalize(&self) -> Result<Self::Normalized, Error> {
        Ok(Run {
            game_id:     p64_from_base36(self.game())?,
            id:          p64_from_base36(self.id())?,
            created:     self.submitted().clone(),
            date:        self.date().clone(),
            category_id: p64_from_base36(self.category())?,
            level_id:    match self.level() {
                None => None,
                Some(level_id) => Some(p64_from_base36(level_id)?),
            },
        })
    }
}
