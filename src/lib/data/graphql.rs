#![warn(clippy::option_unwrap_used, clippy::result_unwrap_used)]

use std::{
    convert::{TryFrom, TryInto},
    sync::Arc,
};

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
    utils::{base36, src_slugify, u64_from_base36},
};

mod global_id;

graphql_schema_from_file!("src/lib/data/graphql/schema.juniper.graphql");

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

#[derive(Debug)]
pub struct Stats {}

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
pub struct CategoryLevel {
    category: DbLinked<db::Category>,
    level:    DbLinked<db::Level>,
}

#[derive(Debug, Clone)]
pub enum Player {
    User(User),
    Guest(String),
}

#[derive(Debug, Clone)]
pub struct Level(DbLinked<db::Level>);

impl StatsFields for Stats {
    fn field_last_updated(&self, executor: &Executor<'_, Context>) -> f64 {
        executor
            .context()
            .database
            .last_updated()
            .timestamp_millis() as f64
    }

    fn field_runs(&self, executor: &Executor<'_, Context>) -> i32 {
        let n = executor.context().database.tables().runs().len();
        n.try_into().expect("impossibly large number of runs")
    }

    fn field_games(&self, executor: &Executor<'_, Context>) -> i32 {
        let n = executor.context().database.tables().games().len();
        n.try_into().expect("impossibly large number of runs")
    }

    fn field_version(&self, _executor: &Executor<'_, Context>) -> String {
        option_env!("CARGO_PKG_VERSION")
            .unwrap_or("unknown")
            .to_string()
    }
}

impl SpeedrunsFields for Speedruns {
    fn field_stats(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Stats, Walked>,
    ) -> Stats {
        Stats {}
    }

    fn field_game(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Game, Walked>,
        slug: String,
    ) -> Option<Game> {
        executor.context().database.game_by_slug(&slug).map(Game)
    }

    fn field_games(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Game, Walked>,
    ) -> Vec<Game> {
        executor.context().database.games().map(Game).collect()
    }

    fn field_run(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
        src_id: ID,
    ) -> Option<Run> {
        let db_id = u64_from_base36(&src_id.to_string());
        match db_id {
            Ok(db_id) => match executor.context().database.run_by_id(db_id) {
                Some(run) => Some(Run(run)),
                None => None,
            },
            Err(_) => None,
        }
    }

    fn field_node(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Node, Walked>,
        id: ID,
    ) -> Option<Node> {
        let database = &executor.context().database;
        match parse_global_id(&id) {
            Ok((id, node_type)) => match node_type {
                NodeType::Game => database.game_by_id(id).map(|g| Node::Game(Game(g))),
                NodeType::Run => database.run_by_id(id).map(|r| Node::Run(Run(r))),
                NodeType::User => database.user_by_id(id).map(|u| Node::User(User(u))),
                NodeType::Level => database.level_by_id(id).map(|l| Node::Level(Level(l))),
                NodeType::Category => database
                    .category_by_id(id)
                    .map(|c| Node::Category(Category(c))),
            },
            Err(_) => None,
        }
    }
}

impl GameFields for Game {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::Game)
    }

    fn field_src_id(&self, _executor: &Executor<'_, Context>) -> String {
        base36(self.0.id)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> &String {
        &self.0.name
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> &String {
        &self.0.slug
    }

    fn field_src_slug(&self, _executor: &Executor<'_, Context>) -> String {
        src_slugify(&self.0.slug)
    }

    fn field_timing_method(&self, _executor: &Executor<'_, Context>) -> TimingMethod {
        match self.0.primary_timing() {
            crate::data::types::TimingMethod::IGT => TimingMethod::Igt,
            crate::data::types::TimingMethod::RTA => TimingMethod::Rta,
            crate::data::types::TimingMethod::RTA_NL => TimingMethod::RtaNl,
        }
    }

    fn field_runs(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Vec<Run> {
        self.0.runs().iter().map(|run| Run(run.clone())).collect()
    }

    fn field_levels(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Level, Walked>,
    ) -> Vec<Level> {
        executor
            .context()
            .database
            .levels_by_game_id(self.0.id)
            .into_iter()
            .map(Level)
            .collect()
    }

    fn field_game_categories(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        executor
            .context()
            .database
            .per_game_categories_by_game_id_and_slug()
            .range((self.0.id, "".to_string())..(self.0.id + 1, "".to_string()))
            .map(|(_key, value)| value)
            .sorted_by(|a, b| (&a.name, a.id).cmp(&(&b.name, b.id)))
            .map(|c| Category(executor.context().database.link(c)))
            .collect()
    }

    fn field_level_categories(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        executor
            .context()
            .database
            .per_level_categories_by_game_id_and_slug()
            .range((self.0.id, "".to_string())..(self.0.id + 1, "".to_string()))
            .map(|(_key, value)| value)
            .sorted_by(|a, b| (&a.name, a.id).cmp(&(&b.name, b.id)))
            .map(|c| Category(executor.context().database.link(c)))
            .collect()
    }
}

impl RunFields for Run {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::Run)
    }

    fn field_src_id(&self, _executor: &Executor<'_, Context>) -> String {
        base36(self.0.id)
    }

    fn field_time_ms(&self, _executor: &Executor<'_, Context>) -> i32 {
        i32::try_from(self.0.time_ms()).expect("impossibly long run")
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

    fn field_videos(&self, _executor: &Executor<'_, Context>) -> Vec<String> {
        self.0.videos().iter().map(|v| v.to_string()).collect()
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
        i32::try_from(*self.0.rank()).expect("impossible number of runs")
    }

    fn field_is_tied(&self, _executor: &Executor<'_, Context>) -> bool {
        *self.0.is_tied()
    }

    fn field_tied_rank(&self, _executor: &Executor<'_, Context>) -> i32 {
        i32::try_from(*self.0.tied_rank()).expect("impossible number of runs")
    }
}

impl ProgressionRunFields for ProgressionRun {
    fn field_run(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Run {
        Run(self.0.run().clone())
    }

    fn field_progress_ms(&self, _executor: &Executor<'_, Context>) -> i32 {
        i32::try_from(*self.0.progress_ms()).expect("impossibly long run")
    }

    fn field_leaderboard_run(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, LeaderboardRun, Walked>,
    ) -> Option<LeaderboardRun> {
        self.0
            .leaderboard_run()
            .as_ref()
            .map(|lr| LeaderboardRun(lr.clone()))
    }
}

impl CategoryFields for Category {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::Category)
    }

    fn field_src_id(&self, _executor: &Executor<'_, Context>) -> String {
        base36(self.0.id)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> &String {
        &self.0.name
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> &String {
        &self.0.slug
    }

    fn field_src_slug(&self, _executor: &Executor<'_, Context>) -> String {
        src_slugify(&self.0.name)
    }

    fn field_leaderboard(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, LeaderboardRun, Walked>,
        level_slug: Option<String>,
        include_obsolete: bool,
        limit: Option<i32>,
    ) -> Vec<LeaderboardRun> {
        let level_id;
        if let Some(level_slug) = level_slug {
            let level = self.0.game().level_by_slug(&level_slug);
            if let Some(level) = level {
                level_id = Some(level.id);
            } else {
                // level specified but not found
                return vec![]
            }
        } else {
            level_id = None;
        }

        let runs = executor
            .context()
            .database
            .runs_by_game_id_and_category_id_and_level_id()
            .get(&(self.0.game_id, self.0.id, level_id));

        if let Some(runs) = runs {
            let runs: Vec<_> = runs
                .iter()
                .map(|run| executor.context().database.link(*run))
                .collect();

            let mut ranked = leaderboard::leaderboard(&runs, include_obsolete);

            if let Some(limit) = limit {
                ranked.truncate(limit.try_into().unwrap_or(0));
            }

            ranked.iter().map(|r| LeaderboardRun(r.clone())).collect()
        } else {
            vec![]
        }
    }

    fn field_progression(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, ProgressionRun, Walked>,
        level_slug: Option<String>,
        _include_ties: bool,
    ) -> Vec<ProgressionRun> {
        let level_id;
        if let Some(level_slug) = level_slug {
            let level = self.0.game().level_by_slug(&level_slug);
            if let Some(level) = level {
                level_id = Some(level.id);
            } else {
                // level specified but not found
                return vec![]
            }
        } else {
            level_id = None;
        }

        let runs = executor
            .context()
            .database
            .runs_by_game_id_and_category_id_and_level_id()
            .get(&(self.0.game_id, self.0.id, level_id));

        if let Some(runs) = runs {
            let runs: Vec<_> = runs
                .iter()
                .map(|run| executor.context().database.link(*run))
                .collect();
            let progress = progression::progression(&runs);

            progress.iter().map(|r| ProgressionRun(r.clone())).collect()
        } else {
            return vec![]
        }
    }

    fn field_levels(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, CategoryLevel, Walked>,
    ) -> Vec<CategoryLevel> {
        executor
            .context()
            .database
            .levels_by_game_id(self.0.game_id)
            .into_iter()
            .map(|level| CategoryLevel {
                category: self.0.clone(),
                level,
            })
            .collect()
    }
}

impl UserFields for User {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::User)
    }

    fn field_src_id(&self, _executor: &Executor<'_, Context>) -> String {
        base36(self.0.id)
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> &String {
        &self.0.slug
    }

    fn field_src_slug(&self, _executor: &Executor<'_, Context>) -> String {
        src_slugify(&self.0.name)
    }
}

impl PlayerFields for Player {
    fn field_name(&self, _executor: &Executor<'_, Context>) -> &String {
        match self {
            Player::User(user) => &user.0.name,
            Player::Guest(name) => &name,
        }
    }

    fn field_user(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> Option<User> {
        match self {
            Player::User(user) => Some(user.clone()),
            Player::Guest(_name) => None,
        }
    }

    fn field_is_guest(&self, _executor: &Executor<'_, Context>) -> bool {
        match self {
            Player::User(_user) => false,
            Player::Guest(_name) => true,
        }
    }
}

impl LevelFields for Level {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        global_id(self.0.id, NodeType::Level)
    }

    fn field_src_id(&self, _executor: &Executor<'_, Context>) -> String {
        base36(self.0.id)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> &String {
        &self.0.name
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> &String {
        &self.0.slug
    }

    fn field_src_slug(&self, _executor: &Executor<'_, Context>) -> String {
        src_slugify(&self.0.name)
    }
}

impl CategoryLevelFields for CategoryLevel {
    fn field_level(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Level, Walked>,
    ) -> Level {
        Level(self.level.clone())
    }

    fn field_category(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Category {
        Category(self.category.clone())
    }

    fn field_leaderboard(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, LeaderboardRun, Walked>,
        include_obsolete: bool,
        limit: Option<i32>,
    ) -> Vec<LeaderboardRun> {
        let runs: Vec<DbLinked<db::Run>> = executor
            .context()
            .database
            .runs_by_game_id_and_category_id_and_level_id()
            .get(&(self.category.game_id, self.category.id, Some(self.level.id)))
            .map(Clone::clone)
            .unwrap_or_else(Default::default)
            .iter()
            .map(|run| executor.context().database.link(*run))
            .collect();

        let mut ranked = leaderboard::leaderboard(&runs, include_obsolete);

        if let Some(limit) = limit {
            ranked.truncate(limit.try_into().unwrap_or(0));
        }

        ranked.iter().map(|r| LeaderboardRun(r.clone())).collect()
    }

    fn field_progression(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, ProgressionRun, Walked>,
        _include_ties: bool,
    ) -> Vec<ProgressionRun> {
        let runs: Vec<DbLinked<db::Run>> = executor
            .context()
            .database
            .runs_by_game_id_and_category_id_and_level_id()
            .get(&(self.category.game_id, self.category.id, Some(self.level.id)))
            .map(Clone::clone)
            .unwrap_or_else(Default::default)
            .iter()
            .map(|run| executor.context().database.link(*run))
            .collect();

        let progress = progression::progression(&runs);

        progress.iter().map(|r| ProgressionRun(r.clone())).collect()
    }
}
