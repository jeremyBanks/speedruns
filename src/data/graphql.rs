use std::{convert::TryFrom, sync::Arc};

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
        leaderboard, types as db,
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
    #[graphql(description = "Get a game.")]
    pub fn game(context: &Context, slug: String) -> FieldResult<Game> {
        match context.database.game_by_slug(&slug) {
            Some(game) => Ok(Game(game)),
            None => Err(FieldError::from("game not found")),
        }
    }

    #[graphql(description = "Get a user.")]
    pub fn user(context: &Context, slug: String) -> FieldResult<User> {
        match context.database.user_by_slug(&slug) {
            Some(user) => Ok(User(user)),
            None => Err(FieldError::from("user not found")),
        }
    }
}

#[derive(Debug, Default)]
pub struct Mutation {}

#[juniper::object(Context = Context)]
#[graphql(description = "Read-write operation root.")]
impl Mutation {
    #[graphql(description = "There are no read-write operations. This is a hack.")]
    pub fn noop(context: &Context) -> FieldResult<RankedRun> {
        Err(FieldError::from("don't call this"))
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

    #[graphql(description = "All of the categories for this game.")]
    pub fn categories(&self, context: &Context) -> FieldResult<Vec<Category>> {
        Err(FieldError::from("not implemented"))
    }

    #[graphql(description = "All of the levels for this game.")]
    pub fn levels(&self, context: &Context) -> FieldResult<Vec<Level>> {
        Err(FieldError::from("not implemented"))
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

#[derive(Debug)]
pub struct RankedRun(leaderboard::RankedRun);

#[juniper::object(Context = Context)]
impl RankedRun {
    pub fn rank(&self, context: &Context) -> FieldResult<i32> {
        Ok(i32::try_from(*self.0.rank()).unwrap())
    }

    pub fn time_ms(&self, context: &Context) -> FieldResult<i32> {
        Ok(i32::try_from(*self.0.time_ms()).unwrap())
    }

    pub fn is_tied(&self, context: &Context) -> FieldResult<bool> {
        Ok(*self.0.is_tied())
    }

    pub fn tied_rank(&self, context: &Context) -> FieldResult<i32> {
        Ok(i32::try_from(*self.0.tied_rank()).unwrap())
    }

    pub fn run(&self, context: &Context) -> FieldResult<Run> {
        Ok(Run(self.0.run().clone()))
    }
}

#[derive(Debug)]
pub struct Category(DbLinked<db::Category>);

#[juniper::object(Context = Context)]
#[graphql(description = "A category for runs of a game on speedrun.com.")]
impl Category {
    #[graphql(description = "The category's base36 ID from speedrun.com.")]
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }
}

#[derive(Debug)]
pub struct User(DbLinked<db::User>);

#[juniper::object(Context = Context)]
#[graphql(description = "A user of speedrun.com.")]
impl User {
    #[graphql(description = "The users's base36 ID from speedrun.com.")]
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }

    #[graphql(description = "The user's URL slug/abbreviation.")]
    pub fn slug(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.slug.to_string())
    }
}

#[derive(Debug)]
pub struct Level(DbLinked<db::Level>);

#[juniper::object(Context = Context)]
#[graphql(description = "A level of a game on speedrun.com.")]
impl Level {
    #[graphql(description = "The level's base36 ID from speedrun.com.")]
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }
}
