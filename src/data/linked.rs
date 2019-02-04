//! Wrapper types for our [crate::data::types] that add an Indices reference.
use std::{fmt::Debug, ops::Deref};

use getset::Getters;
use serde::Serialize;

use crate::data::{base::Indices, types::*};

#[derive(Debug, Clone, Getters)]
pub struct Linked<'db, Model: Debug + Serialize> {
    indices: &'db Indices<'db>,
    model:    &'db Model,
}

impl<'db, Model: Debug + Serialize> Linked<'db, Model> {
    pub fn new(indices: &'db Indices, model: &'db Model) -> Self {
        Self { indices, model }
    }
}

impl<'db, Model: Debug + Serialize> Deref for Linked<'db, Model> {
    type Target = Model;

    fn deref(&self) -> &Model {
        &self.model
    }
}

impl Linked<'_, Run> {
    pub fn game(&self) -> Linked<Game> {
        Linked::new(
            self.indices,
            self.indices
                .games()
                .get(self.game_id())
                .expect("foreign key should be valid"),
        )
    }
}
