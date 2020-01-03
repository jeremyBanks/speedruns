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
/// Read-only operation root.
impl Query {
    /// Get a game.
    pub fn game(context: &Context, slug: String) -> FieldResult<Game> {
        match context.database.game_by_slug(&slug) {
            Some(game) => Ok(Game(game)),
            None => Err(FieldError::from("game not found")),
        }
    }

    /// Get a user.
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
/// Read-write operation root.
impl Mutation {
    /// There are no read-write operations. This is a hack.
    pub fn noop(context: &Context) -> FieldResult<i32> {
        Err(FieldError::from("don't call this"))
    }
}

#[derive(Debug)]
pub struct Game(DbLinked<db::Game>);

#[juniper::object(Context = Context)]
/// A game on speedrun.com.
impl Game {
    /// The game's base36 ID from speedrun.com.
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }

    /// The game's name, in English if possible.
    pub fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.name.to_string())
    }

    /// The game's URL slug/abbreviation.
    pub fn slug(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.slug.to_string())
    }

    /// All of the runs submitted for this game.
    pub fn runs(&self, context: &Context) -> FieldResult<Vec<Run>> {
        Ok(self.0.runs().iter().map(|run| Run(run.clone())).collect())
    }

    /// Returns the ordered ranked runs for a run in a category and optionally level.
    pub fn leaderboard(
        &self,
        context: &Context,
        category: String,
        level: Option<String>,
    ) -> FieldResult<Vec<RankedRun>> {
        let level_id = level.map(|level| self.0.level_by_slug(&level).unwrap().id);
        let category_id = self.0.category_by_slug(&category).unwrap().id;

        let runs: Vec<DbLinked<db::Run>> = self
            .0
            .runs()
            .iter()
            .filter(|run| run.level_id == level_id && run.category_id == category_id)
            .cloned()
            .collect();

        let ranked = leaderboard::rank_runs(&runs);

        Ok(ranked.iter().map(|r| RankedRun(r.clone())).collect())
    }
}

#[derive(Debug)]
pub struct Run(DbLinked<db::Run>);

#[juniper::object(Context = Context)]
/// A run of a game on speedrun.com.
impl Run {
    /// The run's base36 ID from speedrun.com.
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }

    /// The category associated with this run.
    pub fn category(&self, context: &Context) -> FieldResult<Category> {
        Ok(Category(self.0.category()))
    }

    /// The level associated with this run, or null.
    pub fn level(&self, context: &Context) -> FieldResult<Option<Level>> {
        Ok(self.0.level().map(Level))
    }
}

#[derive(Debug)]
pub struct RankedRun(leaderboard::RankedRun);

#[juniper::object(Context = Context)]
impl RankedRun {
    /// This run's rank, with ties broken by date.
    pub fn rank(&self, context: &Context) -> FieldResult<i32> {
        Ok(i32::try_from(*self.0.rank()).unwrap())
    }

    /// The time of this run, as measured by this leaderboard's rules, in miliseconds.
    pub fn time_ms(&self, context: &Context) -> FieldResult<i32> {
        Ok(i32::try_from(*self.0.time_ms()).unwrap())
    }

    /// Whether this run is tied for this rank.
    pub fn is_tied(&self, context: &Context) -> FieldResult<bool> {
        Ok(*self.0.is_tied())
    }

    /// This run's rank, with ties unbroken.
    pub fn tied_rank(&self, context: &Context) -> FieldResult<i32> {
        Ok(i32::try_from(*self.0.tied_rank()).unwrap())
    }

    /// The run.
    pub fn run(&self, context: &Context) -> FieldResult<Run> {
        Ok(Run(self.0.run().clone()))
    }
}

#[derive(Debug)]
pub struct Category(DbLinked<db::Category>);

#[juniper::object(Context = Context)]
/// A category for runs of a game on speedrun.com.
impl Category {
    /// The category's base36 ID from speedrun.com.
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }

    /// The category's name.
    pub fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.name.clone())
    }

    /// The category's slug.
    pub fn slug(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.slug.clone())
    }
}

#[derive(Debug)]
pub struct User(DbLinked<db::User>);

#[juniper::object(Context = Context)]
/// A user of speedrun.com.
impl User {
    /// The users's base36 ID from speedrun.com.
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }

    /// The user's URL slug/abbreviation.
    pub fn slug(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.slug.clone())
    }
}

#[derive(Debug)]
pub struct Level(DbLinked<db::Level>);

#[juniper::object(Context = Context)]
/// A level of a game on speedrun.com.
impl Level {
    /// The level's base36 ID from speedrun.com.
    pub fn id(&self, context: &Context) -> FieldResult<String> {
        Ok(base36(self.0.id))
    }

    /// The level's name.
    pub fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.name.clone())
    }

    /// The level's slug.
    pub fn slug(&self, context: &Context) -> FieldResult<String> {
        Ok(self.0.slug.clone())
    }

    /// The associated game.
    pub fn game(&self, context: &Context) -> FieldResult<Game> {
        Ok(Game(self.0.game()))
    }

    /// Returns ordered ranked runs.
    pub fn leaderboard(
        &self,
        context: &Context,
        category: String,
    ) -> FieldResult<Vec<RankedRun>> {
        let game = self.0.game();

        let category_id = game.category_by_slug(&category).unwrap().id;

        let runs: Vec<DbLinked<db::Run>> = game
            .runs()
            .iter()
            .filter(|run| run.level_id == Some(self.0.id) && run.category_id == category_id)
            .cloned()
            .collect();

        let ranked = leaderboard::rank_runs(&runs);

        Ok(ranked.iter().map(|r| RankedRun(r.clone())).collect())
    }
}
