#![warn(
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

use std::{convert::TryFrom, sync::Arc};

use itertools::Itertools;
#[allow(unused)]
use juniper::{
    graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value,
    object, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue,
    ScalarValue,
};
use juniper::{Executor, ID};
use juniper_from_schema::graphql_schema_from_file;

use crate::{
    data::{
        database::{Database, Linked as DbLinked},
        graphql::global_id::{global_id, parse_global_id, NodeType},
        leaderboard, progression, types as db,
    },
    utils::{base36, src_slugify},
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
pub struct LeaderboardRun(leaderboard::LeaderboardRun);

#[derive(Debug, Clone)]
pub struct ProgressionRun(progression::ProgressionRun);

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
        slug: String,
    ) -> Option<Game> {
        match executor.context().database.game_by_slug(&slug) {
            Some(game) => Some(Game(game)),
            None => None,
        }
    }
}

impl GameFields for Game {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::Game)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> String {
        self.0.name.clone()
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.slug.clone())
    }

    fn field_src_slug(&self, _executor: &Executor<'_, Context>) -> String {
        src_slugify(&self.0.slug)
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
        // TODO: not a full table scan
        executor
            .context()
            .database
            .levels()
            .filter(|level| level.game_id == self.0.id)
            .sorted_by(|a, b| a.name.cmp(&b.name))
            .map(Level)
            .collect()
    }

    fn field_game_categories(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        // TODO: not a full table scan
        (executor
            .context()
            .database
            .categories()
            .filter(|category| {
                category.game_id == self.0.id && category.per == db::CategoryType::PerGame
            })
            .sorted_by(|a, b| (&a.name, a.id).cmp(&(&b.name, b.id)))
            .map(Category)
            .collect())
    }

    fn field_level_categories(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        // TODO: not a full table scan
        (executor
            .context()
            .database
            .categories()
            .filter(|category| {
                category.game_id == self.0.id && category.per == db::CategoryType::PerLevel
            })
            .sorted_by(|a, b| (&a.name, a.id).cmp(&(&b.name, b.id)))
            .map(Category)
            .collect())
    }
}

impl RunFields for Run {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::Run)
    }

    fn field_time_ms(&self, _executor: &Executor<'_, Context>) -> i32 {
        (i32::try_from(self.0.time_ms().expect("must have primary timing"))
            .expect("impossibly long wrong"))
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
                    let user = executor
                        .context()
                        .database
                        .user_by_id(*user_id)
                        .expect("database integrity");
                    Player::User(User(user))
                }
                db::RunPlayer::GuestName(name) => Player::Guest(name.clone()),
            })
            .collect()
    }
}

impl LeaderboardRunFields for LeaderboardRun {
    fn field_run(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Run {
        Run(self.0.run().clone())
    }

    fn field_rank(&self, _executor: &Executor<'_, Context>) -> i32 {
        (i32::try_from(*self.0.rank()).expect("impossible number of runs"))
    }

    fn field_is_tied(&self, _executor: &Executor<'_, Context>) -> bool {
        (*self.0.is_tied())
    }

    fn field_tied_rank(&self, _executor: &Executor<'_, Context>) -> i32 {
        (i32::try_from(*self.0.tied_rank()).expect("impossible number of runs"))
    }
}

impl ProgressionRunFields for ProgressionRun {
    fn field_run(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Run {
        (Run(self.0.run().clone()))
    }

    fn field_improvement_ms(&self, _executor: &Executor<'_, Context>) -> i32 {
        (i32::try_from(*self.0.improvement_ms()).expect("impossibly long wrong"))
    }

    fn field_leaderboard_run(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, LeaderboardRun, Walked>,
    ) -> Option<LeaderboardRun> {
        Some(LeaderboardRun(self.0.leaderboard_run().clone()))
    }
}

impl CategoryFields for Category {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::Category)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.name.clone())
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.slug.clone())
    }

    fn field_src_slug(&self, _executor: &Executor<'_, Context>) -> String {
        src_slugify(&self.0.name)
    }

    fn field_leaderboard(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, LeaderboardRun, Walked>,
        level_slug: Option<String>,
        _include_obsolete: bool,
    ) -> Vec<LeaderboardRun> {
        let level_id = level_slug.map(|level_slug| {
            self.0
                .game()
                .level_by_slug(&level_slug)
                .expect("level not found")
                .id
        });
        let runs: Vec<DbLinked<db::Run>> = self
            .0
            .runs()
            .iter()
            .filter(|run| run.level_id == level_id && run.category_id == self.0.id)
            .cloned()
            .collect();

        let ranked = leaderboard::leaderboard(&runs);

        (ranked.iter().map(|r| LeaderboardRun(r.clone())).collect())
    }

    fn field_progression(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, ProgressionRun, Walked>,
        level_slug: Option<String>,
        _include_ties: bool,
    ) -> Vec<ProgressionRun> {
        let level_id = level_slug.map(|level_slug| {
            self.0
                .game()
                .level_by_slug(&level_slug)
                .expect("level not found")
                .id
        });
        let runs: Vec<DbLinked<db::Run>> = self
            .0
            .runs()
            .iter()
            .filter(|run| run.level_id == level_id && run.category_id == self.0.id)
            .cloned()
            .collect();

        let progress = progression::progression(&runs);

        (progress.iter().map(|r| ProgressionRun(r.clone())).collect())
    }
}

impl UserFields for User {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::User)
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.slug.clone())
    }

    fn field_src_slug(&self, _executor: &Executor<'_, Context>) -> String {
        src_slugify(&self.0.name)
    }
}

impl PlayerFields for Player {
    fn field_name(&self, _executor: &Executor<'_, Context>) -> String {
        (match self {
            Player::User(user) => user.0.name.clone(),
            Player::Guest(name) => name.clone(),
        })
    }

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

    fn field_is_guest(&self, _executor: &Executor<'_, Context>) -> bool {
        (match self {
            Player::User(_user) => false,
            Player::Guest(_name) => true,
        })
    }
}

impl LevelFields for Level {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::Level)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.name.clone())
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> String {
        (self.0.slug.clone())
    }

    fn field_src_slug(&self, _executor: &Executor<'_, Context>) -> String {
        src_slugify(&self.0.name)
    }

    fn field_categories(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        // TODO: not a full table scan
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

    fn field_leaderboard(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, LeaderboardRun, Walked>,
        category_slug: Option<String>,
        _include_obsolete: bool,
    ) -> Vec<LeaderboardRun> {
        let category_id = category_slug.map(|category_slug| {
            self.0
                .game()
                .per_level_category_by_slug(&category_slug)
                .expect("category not found")
                .id
        });
        let runs: Vec<DbLinked<db::Run>> = self
            .0
            .game()
            .runs()
            .iter()
            .filter(|run| {
                Some(run.category_id) == category_id && run.level_id == Some(self.0.id)
            })
            .cloned()
            .collect();

        let ranked = leaderboard::leaderboard(&runs);

        (ranked.iter().map(|r| LeaderboardRun(r.clone())).collect())
    }

    fn field_progression(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, ProgressionRun, Walked>,
        category_slug: Option<String>,
        _include_ties: bool,
    ) -> Vec<ProgressionRun> {
        let category_id = category_slug.map(|category_slug| {
            self.0
                .game()
                .per_level_category_by_slug(&category_slug)
                .expect("category not found")
                .id
        });
        let runs: Vec<DbLinked<db::Run>> = self
            .0
            .game()
            .runs()
            .iter()
            .filter(|run| {
                Some(run.category_id) == category_id && run.level_id == Some(self.0.id)
            })
            .cloned()
            .collect();

        let progress = progression::progression(&runs);

        (progress.iter().map(|r| ProgressionRun(r.clone())).collect())
    }
}
