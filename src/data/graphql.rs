use std::sync::Arc;

use juniper::{FieldError, FieldResult, RootNode};

#[allow(unused)]
use juniper::{
    graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value,
    object, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue,
    ScalarValue,
};

use crate::{
    data::{
        database::{Database, Linked as DbLinked},
        types as db,
    },
    utils::base36,
};

#[derive(Debug)]
pub struct Context {
    pub database: Arc<Database>,
}
impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

#[derive(Debug, Default)]
pub struct Query {}

#[juniper::object(Context = Context)]
#[graphql(description = "Read-only operation root.")]
impl Query {
    #[graphql(description = "
        Get a game by id or slug.
    ")]
    pub fn game(context: &Context, slug: String) -> FieldResult<Game> {
        match context.database.game_by_slug(&slug) {
            Some(game) => Ok(Game(game)),
            None => Err(FieldError::from("not found")),
        }
    }
}

#[derive(Debug, Default)]
pub struct Mutation {}

#[juniper::object(Context = Context)]
#[graphql(description = "Read-write operation root.")]
impl Mutation {
    #[graphql(description = "No-op workaround for https://git.io/JeNXr.")]
    pub fn noop(context: &Context) -> FieldResult<bool> {
        Ok(false)
    }
}

#[derive(Debug)]
pub struct Game(DbLinked<db::Game>);

#[juniper::object(Context = Context)]
#[graphql(description = "A game on speedrun.com.")]
impl Game {
    #[graphql(description = "The game's base36 ID from speedrun.com.")]
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }

    #[graphql(description = "The game's name, in English if possible.")]
    pub fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.name.to_string())
    }

    #[graphql(description = "The game's URL slug/abbreviation.")]
    pub fn slug(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.slug.to_string())
    }

    #[graphql(description = "All of the runs submitted for this game.")]
    pub fn runs(&self, context: &Context) -> FieldResult<Vec<Run>> {
        let runs = context.database.runs_by_game_id(self.0.id).unwrap();
        Ok(runs.iter().map(|run| Run(run.clone())).collect())
    }
}

#[derive(Debug)]
pub struct Run(DbLinked<db::Run>);

#[juniper::object(Context = Context)]
#[graphql(description = "A run of a game on speedrun.com.")]
impl Run {
    #[graphql(description = "The run's base36 ID from speedrun.com.")]
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }
}
