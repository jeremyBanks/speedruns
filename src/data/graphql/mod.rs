use std::{convert::TryFrom, sync::Arc};

#[allow(unused)]
use juniper::{
    graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value,
    object, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue,
    ScalarValue,
};
use juniper::{Executor, FieldResult, ID};
use juniper_from_schema::graphql_schema_from_file;

use crate::{
    data::{
        database::{Database, Linked as DbLinked},
        leaderboard, types as db,
    },
    utils::{base36, u64_from_base36},
};

mod global_id;

graphql_schema_from_file!("public/graphql/schema.graphql");

pub fn schema() -> Schema {
    Schema::new(Speedruns {}, Speedruns {})
}

#[derive(Debug, Clone)]
pub struct Context {
    pub database: Arc<Database>,
}

impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Speedruns {}

#[derive(Debug, Clone)]
pub struct Game(DbLinked<db::Game>);

#[derive(Debug, Clone)]
pub struct Run(DbLinked<db::Run>);

#[derive(Debug, Clone)]
pub struct RankedRun(leaderboard::RankedRun);

#[derive(Debug, Clone)]
pub struct Category(DbLinked<db::Category>);

#[derive(Debug, Clone)]
pub struct User(DbLinked<db::User>);

#[derive(Debug, Clone)]
pub enum Player {
    User(User),
    Guest(String),
}

#[derive(Debug, Clone)]
pub struct Level(DbLinked<db::Level>);
impl SpeedrunsFields for Speedruns {
    fn field_game(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Game, Walked>,
        slug: Option<String>,
        id: Option<String>,
    ) -> FieldResult<Option<Game>> {
        let _todo = id;
        let slug = slug.unwrap();
        match executor.context().database.game_by_slug(&slug) {
            Some(game) => Ok(Some(Game(game))),
            None => Ok(None),
        }
    }

    fn field_user(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
        slug: Option<String>,
        id: Option<String>,
    ) -> FieldResult<Option<User>> {
        let _todo = id;
        let slug = slug.unwrap();
        match executor.context().database.user_by_slug(&slug) {
            Some(user) => Ok(Some(User(user))),
            None => Ok(None),
        }
    }

    fn field_run(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
        id: String,
    ) -> Option<Run> {
        let id = u64_from_base36(&id).unwrap();
        match executor.context().database.run_by_id(id) {
            Some(run) => (Some(Run(run))),
            None => (None),
        }
    }

    fn field_node(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Node, Walked>,
        _id: ID,
    ) -> Option<Node> {
        None
    }
}

impl GameFields for Game {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        ID::from(base36(self.0.id))
    }

    fn field_src_id(&self, _executor: &Executor<'_, Context>) -> String {
        base36(self.0.id)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> String {
        self.0.name.to_string()
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.slug.to_string())
    }

    fn field_runs(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Vec<Run> {
        (self.0.runs().iter().map(|run| Run(run.clone())).collect())
    }

    fn field_levels(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Level, Walked>,
    ) -> Vec<Level> {
        // XXX: full table scan
        executor
            .context()
            .database
            .levels()
            .filter(|level| level.game_id == self.0.id)
            .map(Level)
            .collect()
    }

    // Full-game run categories.
    fn field_categories(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        // XXX: full table scan
        (executor
            .context()
            .database
            .categories()
            .filter(|category| {
                category.game_id == self.0.id && category.per == db::CategoryType::PerGame
            })
            .map(Category)
            .collect())
    }

    fn field_leaderboard(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, RankedRun, Walked>,
        category: String,
        level: Option<String>,
    ) -> Vec<RankedRun> {
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

        (ranked.iter().map(|r| RankedRun(r.clone())).collect())
    }

    fn field_run(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
        id: String,
    ) -> Option<Run> {
        let id = u64_from_base36(&id).unwrap();
        match executor.context().database.run_by_id(id) {
            Some(run) =>
                if run.game_id == self.0.id {
                    Some(Run(run))
                } else {
                    None
                },
            None => None,
        }
    }
}

impl RunFields for Run {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> String {
        base36(self.0.id)
    }

    fn field_game(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Game, Walked>,
    ) -> Game {
        Game(self.0.game())
    }

    fn field_category(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Category {
        Category(self.0.category())
    }

    fn field_level(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Level, Walked>,
    ) -> Option<Level> {
        self.0.level().map(Level)
    }

    fn field_date(&self, _executor: &Executor<'_, Context>) -> Option<f64> {
        // not sure if this cast is potentially lossy in practice
        self.0
            .date()
            .map(|c| c.and_hms(12, 8, 4).timestamp() as f64)
    }

    fn field_players(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Player, Walked>,
    ) -> Vec<Player> {
        self.0
            .players()
            .iter()
            .map(|run_player| match run_player {
                db::RunPlayer::UserId(user_id) => {
                    let user = executor.context().database.user_by_id(*user_id).unwrap();
                    Player::User(User(user))
                }
                db::RunPlayer::GuestName(name) => Player::Guest(name.clone()),
            })
            .collect()
    }
}

impl RankedRunFields for RankedRun {
    /// This run's rank, with ties broken by date.
    fn field_rank(&self, _executor: &Executor<'_, Context>) -> i32 {
        (i32::try_from(*self.0.rank()).unwrap())
    }

    /// The time of this run, as measured by this leaderboard's rules, in miliseconds.
    fn field_time_ms(&self, _executor: &Executor<'_, Context>) -> i32 {
        (i32::try_from(*self.0.time_ms()).unwrap())
    }

    /// Whether this run is tied for this rank.
    fn field_is_tied(&self, _executor: &Executor<'_, Context>) -> bool {
        (*self.0.is_tied())
    }

    /// This run's rank, with ties unbroken.
    fn field_tied_rank(&self, _executor: &Executor<'_, Context>) -> i32 {
        (i32::try_from(*self.0.tied_rank()).unwrap())
    }

    /// The run.
    fn field_run(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Run {
        (Run(self.0.run().clone()))
    }
}

impl CategoryFields for Category {
    /// The category's base36 ID from speedrun.com.
    fn field_id(&self, _executor: &Executor<'_, Context>) -> String {
        (base36(self.0.id))
    }

    /// The category's name.
    fn field_name(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.name.clone())
    }

    /// The category's slug.
    fn field_slug(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.slug.clone())
    }
}

impl UserFields for User {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> String {
        (base36(self.0.id))
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.slug.clone())
    }
}

impl PlayerFields for Player {
    /// The player's name, which may be a distinct username or a non-distinct guest
    /// nickname.
    fn field_name(&self, _executor: &Executor<'_, Context>) -> String {
        (match self {
            Player::User(user) => user.0.name.clone(),
            Player::Guest(name) => name.clone(),
        })
    }

    /// The associated user, if this is a user.
    fn field_user(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> Option<User> {
        (match self {
            Player::User(user) => Some(user.clone()),
            Player::Guest(_name) => None,
        })
    }

    /// Whether this player is a guest instead of a user.
    fn field_is_guest(&self, _executor: &Executor<'_, Context>) -> bool {
        (match self {
            Player::User(_user) => false,
            Player::Guest(_name) => true,
        })
    }
}

impl LevelFields for Level {
    /// The level's base36 ID from speedrun.com.
    fn field_id(&self, _executor: &Executor<'_, Context>) -> String {
        (base36(self.0.id))
    }

    /// The level's name.
    fn field_name(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.name.clone())
    }

    /// The level's slug.
    fn field_slug(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.slug.clone())
    }

    /// The associated game.
    fn field_game(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Game, Walked>,
    ) -> Game {
        (Game(self.0.game()))
    }

    // Individual level run categories.
    fn field_categories(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        // XXX: full table scan
        (executor
            .context()
            .database
            .categories()
            .filter(|category| {
                category.game_id == self.0.id && category.per == db::CategoryType::PerLevel
            })
            .map(Category)
            .collect())
    }

    /// Returns ordered ranked runs.
    fn field_leaderboard(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, RankedRun, Walked>,
        category: String,
    ) -> Vec<RankedRun> {
        let game = self.0.game();

        let category_id = game.category_by_slug(&category).unwrap().id;

        let runs: Vec<DbLinked<db::Run>> = game
            .runs()
            .iter()
            .filter(|run| run.level_id == Some(self.0.id) && run.category_id == category_id)
            .cloned()
            .collect();

        let ranked = leaderboard::rank_runs(&runs);

        (ranked.iter().map(|r| RankedRun(r.clone())).collect())
    }
}
