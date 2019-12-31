use std::{num::NonZeroU64 as Id64, sync::Arc};

use juniper::{FieldError, FieldResult, RootNode};

#[allow(unused)]
use juniper::{
    graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value,
    object, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue,
    ScalarValue,
};

use crate::{
    data::database::Database,
    utils::{base36, id64_from_base36},
};

#[derive(Debug)]
pub struct Context {
    pub database: Arc<Database>,
}
impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Game {
    id: Id64,
}

#[juniper::object(Context = Context)]
#[graphql(description = "A game on speedrun.com.")]
impl Game {
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        let game = context.database.game_by_id(self.id).unwrap();
        Ok(base36(game.id))
    }

    pub fn name(&self, context: &Context) -> FieldResult<String> {
        let game = context.database.game_by_id(self.id).unwrap();
        Ok(game.name.to_string())
    }

    pub fn slug(&self, context: &Context) -> FieldResult<String> {
        let game = context.database.game_by_id(self.id).unwrap();
        Ok(game.slug.to_string())
    }
}

#[derive(Debug, Default)]
pub struct Query {}

#[juniper::object(Context = Context)]
impl Query {
    #[graphql(description = "
        Get a game by id or slug.
    ")]
    pub fn game(context: &Context, slug: String) -> FieldResult<Game> {
        match context.database.game_by_slug(&slug) {
            Some(game) => Ok(Game { id: game.id }),
            None => Err(FieldError::from("not found")),
        }
    }
}

#[derive(Debug, Default)]
pub struct Mutation {}

#[juniper::object(Context = Context)]
impl Mutation {
    #[graphql(description = "No-op workaround for https://git.io/JeNXr.")]
    pub fn noop(context: &Context) -> FieldResult<bool> {
        Ok(false)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
