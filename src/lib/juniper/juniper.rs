#![warn(clippy::option_unwrap_used, clippy::result_unwrap_used)]

use std::{
    convert::{TryFrom, TryInto},
    sync::Arc,
};

use derive_more::{Deref, From, Into};
use getset::Getters;
use itertools::Itertools;
#[allow(unused)]
use juniper::{
    graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value,
    object, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue,
    ScalarValue,
};
use juniper::{Executor, ID};
use juniper_from_schema::graphql_schema_from_file;

use speedruns_database::Database;
use speedruns_models::{
    self as models,
    aggregation::{leaderboard::leaderboard, progression::progression},
};
use speedruns_utils::{base36, slugify, u64_from_base36};

pub mod cli;

mod global_id;
use global_id::{global_id, parse_global_id, NodeType};

graphql_schema_from_file!("./schema.juniper.graphql");

pub fn schema() -> Schema {
    Schema::new(Speedruns {}, Speedruns {})
}

#[derive(Debug, Deref, Getters)]
#[get = "pub"]
pub struct Context {
    pub database: Arc<Database>,
}

impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Speedruns {}

#[derive(Debug)]
pub struct Stats {}

#[derive(Debug, Deref, From, Into)]
pub struct Game(models::Game);

#[derive(Debug, Deref, From, Into)]
pub struct Category(models::Category);

#[derive(Debug, Deref, From, Into)]
pub struct Level(models::Level);

#[derive(Debug, Deref, From, Into)]
pub struct Run(models::Run);

#[derive(Debug, Deref, From, Into)]
pub struct User(models::User);

#[derive(Debug, Deref, From, Into)]
pub struct LeaderboardRun(models::aggregation::leaderboard::LeaderboardRun);

#[derive(Debug, Deref, From, Into)]
pub struct ProgressionRun(models::aggregation::progression::ProgressionRun);

#[derive(Debug, Getters)]
#[get = "pub"]
pub struct CategoryLevel {
    category: Category,
    level: Level,
}

#[derive(Debug)]
pub enum Player {
    User(User),
    Guest(String),
}

impl StatsFields for Stats {
    fn field_last_updated(&self, executor: &Executor<'_, Context>) -> f64 {
        executor
            .context()
            .indicies()
            .last_updated()
            .timestamp_millis() as f64
    }

    fn field_runs(&self, executor: &Executor<'_, Context>) -> i32 {
        let n = executor.context().database.runs().len();
        n.try_into().expect("impossibly large number of runs")
    }

    fn field_games(&self, executor: &Executor<'_, Context>) -> i32 {
        let n = executor.context().database.games().len();
        n.try_into().expect("impossibly large number of runs")
    }

    fn field_version(&self, executor: &Executor<'_, Context>) -> String {
        option_env!("CARGO_PKG_VERSION")
            .unwrap_or("unknown")
            .to_string()
    }
}

impl SpeedrunsFields for Speedruns {
    fn field_stats(
        &self,
        executor: &Executor<'_, Context>,
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
        executor
            .context()
            .indicies()
            .games_by_slug()
            .get(&slug[..])
            .map(|game| (*game).clone().into())
    }

    fn field_games(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Game, Walked>,
    ) -> Vec<Game> {
        executor
            .context()
            .games()
            .values()
            .map(|game| game.clone().into())
            .collect()
    }

    fn field_run(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
        src_id: ID,
    ) -> Option<Run> {
        let db_id = u64_from_base36(&src_id.to_string());
        match db_id {
            Ok(db_id) => match executor.context().database.runs().get(&db_id) {
                Some(run) => Some((*run).clone().into()),
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
                NodeType::Game => database
                    .games()
                    .get(&id)
                    .map(|g| Node::Game((*g).clone().into())),
                NodeType::Run => database
                    .runs()
                    .get(&id)
                    .map(|r| Node::Run((*r).clone().into())),
                NodeType::User => database
                    .users()
                    .get(&id)
                    .map(|u| Node::User((*u).clone().into())),
                NodeType::Level => database
                    .levels()
                    .get(&id)
                    .map(|l| Node::Level(l.clone().into())),
                NodeType::Category => database
                    .categories()
                    .get(&id)
                    .map(|c| Node::Category(c.clone().into())),
            },
            Err(_) => None,
        }
    }

    fn field_seed(&self, executor: &Executor<'_, Context>) -> i32 {
        rand::Rng::gen(&mut rand::thread_rng())
    }
}

impl GameFields for Game {
    fn field_id(&self, executor: &Executor<'_, Context>) -> ID {
        global_id(*self.id(), NodeType::Game)
    }

    fn field_src_id(&self, executor: &Executor<'_, Context>) -> String {
        base36(*self.id())
    }

    fn field_name(&self, executor: &Executor<'_, Context>) -> &String {
        self.name()
    }

    fn field_slug(&self, executor: &Executor<'_, Context>) -> &String {
        self.slug()
    }

    fn field_timing_method(&self, executor: &Executor<'_, Context>) -> TimingMethod {
        match self.primary_timing() {
            speedruns_models::TimingMethod::IGT => TimingMethod::Igt,
            speedruns_models::TimingMethod::RTA => TimingMethod::Rta,
            speedruns_models::TimingMethod::RTA_NL => TimingMethod::RtaNl,
        }
    }

    fn field_runs(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Vec<Run> {
        executor
            .context()
            .indicies()
            .runs_by_game_id_and_category_id_and_level_id()
            .range((*self.id(), 0, None)..(*self.id() + 1, 0, None))
            .map(|(_key, value)| value)
            .flatten()
            .map(|run| (*run).clone().into())
            .collect()
    }

    fn field_levels(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Level, Walked>,
    ) -> Vec<Level> {
        executor
            .context()
            .indicies()
            .levels_by_game_id_and_slug()
            .range((*self.id(), "")..(*self.id() + 1, ""))
            .map(|(_key, level)| (*level).clone().into())
            .collect()
    }

    fn field_game_categories(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        executor
            .context()
            .indicies()
            .per_game_categories_by_game_id_and_slug()
            .range((*self.id(), "")..(*self.id() + 1, ""))
            .map(|(_key, value)| value)
            .sorted_by(|a, b| (&a.name, a.id).cmp(&(&b.name, b.id)))
            .map(|c| Category((*c).clone().into()))
            .collect()
    }

    fn field_level_categories(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        executor
            .context()
            .indicies()
            .per_level_categories_by_game_id_and_slug()
            .range((*self.id(), "")..(*self.id() + 1, ""))
            .map(|(_key, value)| value)
            .sorted_by(|a, b| (&a.name, a.id).cmp(&(&b.name, b.id)))
            .map(|c| Category((*c).clone().into()))
            .collect()
    }
}

impl RunFields for Run {
    fn field_id(&self, executor: &Executor<'_, Context>) -> ID {
        global_id(*self.id(), NodeType::Run)
    }

    fn field_src_id(&self, executor: &Executor<'_, Context>) -> String {
        base36(*self.id())
    }

    fn field_time_ms(&self, executor: &Executor<'_, Context>) -> i32 {
        let game = &executor.context().games()[self.game_id()];
        i32::try_from(
            self.times_ms()
                .get(game.primary_timing())
                .expect("missing primary timing"),
        )
        .expect("impossibly long run")
    }

    fn field_category(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Category {
        (&executor.context().categories()[self.category_id()])
            .clone()
            .into()
    }

    fn field_level(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Level, Walked>,
    ) -> Option<Level> {
        self.level_id()
            .map(|level_id| (&executor.context().levels()[&level_id]).clone().into())
    }

    fn field_date(&self, executor: &Executor<'_, Context>) -> Option<f64> {
        // not sure if this cast is potentially lossy in practice
        self.date().map(|c| c.and_hms(12, 8, 4).timestamp() as f64)
    }

    fn field_players(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Player, Walked>,
    ) -> Vec<Player> {
        self.players()
            .iter()
            .map(|run_player| match run_player {
                models::RunPlayer::UserId(user_id) => {
                    let user = executor
                        .context()
                        .database
                        .users()
                        .get(user_id)
                        .expect("database integrity");
                    Player::User(user.clone().into())
                }
                models::RunPlayer::GuestName(name) => Player::Guest(name.clone()),
            })
            .collect()
    }

    fn field_videos(&self, executor: &Executor<'_, Context>) -> Vec<String> {
        self.videos().iter().map(|v| v.to_string()).collect()
    }
}

impl LeaderboardRunFields for LeaderboardRun {
    fn field_run(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Run {
        Run(self.run().clone())
    }

    fn field_rank(&self, executor: &Executor<'_, Context>) -> i32 {
        i32::try_from(*self.rank()).expect("impossible number of runs")
    }

    fn field_is_tied(&self, executor: &Executor<'_, Context>) -> bool {
        *self.is_tied()
    }

    fn field_tied_rank(&self, executor: &Executor<'_, Context>) -> i32 {
        i32::try_from(*self.tied_rank()).expect("impossible number of runs")
    }
}

impl ProgressionRunFields for ProgressionRun {
    fn field_run(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Run {
        Run(self.run().clone())
    }

    fn field_progress_ms(&self, executor: &Executor<'_, Context>) -> i32 {
        i32::try_from(*self.progress_ms()).expect("impossibly long run")
    }

    fn field_leaderboard_run(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, LeaderboardRun, Walked>,
    ) -> Option<LeaderboardRun> {
        self.leaderboard_run()
            .as_ref()
            .map(|lr| LeaderboardRun(lr.clone()))
    }
}

impl CategoryFields for Category {
    fn field_id(&self, executor: &Executor<'_, Context>) -> ID {
        global_id(*self.id(), NodeType::Category)
    }

    fn field_src_id(&self, executor: &Executor<'_, Context>) -> String {
        base36(*self.id())
    }

    fn field_name(&self, executor: &Executor<'_, Context>) -> &String {
        &*self.name()
    }

    fn field_slug(&self, executor: &Executor<'_, Context>) -> String {
        slugify(&*self.name())
    }

    fn field_leaderboard(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, LeaderboardRun, Walked>,
        level_slug: Option<String>,
        include_obsolete: bool,
        limit: Option<i32>,
    ) -> Vec<LeaderboardRun> {
        let game = &executor.context().games()[self.game_id()];
        let level_id;
        if let Some(level_slug) = level_slug {
            let level = executor
                .context()
                .indicies()
                .levels_by_game_id_and_slug()
                .get(&(*self.game_id(), &level_slug));
            if let Some(level) = level {
                level_id = Some(level.id);
            } else {
                // level specified but not found
                return vec![];
            }
        } else {
            level_id = None;
        }

        let runs = executor
            .context()
            .indicies()
            .runs_by_game_id_and_category_id_and_level_id()
            .get(&(*self.game_id(), *self.id(), level_id));

        if let Some(runs) = runs {
            let runs: Vec<_> = runs.iter().map(|run| (*run).clone()).collect();

            let mut ranked = leaderboard(&game, runs.iter(), include_obsolete);

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
        let game = &executor.context().games()[self.game_id()];
        let level_id;
        if let Some(level_slug) = level_slug {
            let level = executor
                .context()
                .indicies()
                .levels_by_game_id_and_slug()
                .get(&(*self.game_id(), &level_slug));
            if let Some(level) = level {
                level_id = Some(level.id);
            } else {
                // level specified but not found
                return vec![];
            }
        } else {
            level_id = None;
        }

        let runs: Vec<models::Run> = match level_id {
            Some(level_id) => match executor
                .context()
                .indicies()
                .runs_by_game_id_and_category_id_and_level_id()
                .get(&(*self.game_id(), *self.id(), Some(level_id)))
            {
                Some(runs) => runs.iter().map(|run| (*run).clone()).collect(),
                None => Vec::new(),
            },
            None => executor
                .context()
                .indicies()
                .runs_by_game_id_and_category_id_and_level_id()
                .range(
                    &(*self.game_id(), *self.id(), None)
                        ..&(*self.game_id() + 1, *self.id(), None),
                )
                .map(|(_key, value)| value)
                .map(|x| x.iter())
                .flatten()
                .map(|run| (*run).clone().into())
                .collect(),
        };

        let progress = progression(&game, runs.iter());
        progress.iter().map(|r| ProgressionRun(r.clone())).collect()
    }

    fn field_levels(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, CategoryLevel, Walked>,
    ) -> Vec<CategoryLevel> {
        executor
            .context()
            .indicies()
            .levels_by_game_id_and_slug()
            .range((*self.id(), "")..(*self.id() + 1, ""))
            .map(|(_key, level)| CategoryLevel {
                category: (*self).clone().into(),
                level: (*level).clone().into(),
            })
            .collect()
    }
}

impl UserFields for User {
    fn field_id(&self, executor: &Executor<'_, Context>) -> ID {
        global_id(*self.id(), NodeType::User)
    }

    fn field_src_id(&self, executor: &Executor<'_, Context>) -> String {
        base36(*self.id())
    }

    fn field_slug(&self, executor: &Executor<'_, Context>) -> String {
        slugify(&*self.name())
    }
}

impl PlayerFields for Player {
    fn field_name(&self, executor: &Executor<'_, Context>) -> &String {
        match self {
            Player::User(user) => &user.name,
            Player::Guest(name) => &name,
        }
    }

    fn field_user(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> Option<User> {
        match self {
            Player::User(user) => Some((*user).clone().into()),
            Player::Guest(_name) => None,
        }
    }

    fn field_is_guest(&self, executor: &Executor<'_, Context>) -> bool {
        match self {
            Player::User(_user) => false,
            Player::Guest(_name) => true,
        }
    }
}

impl LevelFields for Level {
    fn field_id(&self, executor: &Executor<'_, Context>) -> ID {
        global_id(*self.id(), NodeType::Level)
    }

    fn field_src_id(&self, executor: &Executor<'_, Context>) -> String {
        base36(*self.id())
    }

    fn field_name(&self, executor: &Executor<'_, Context>) -> &String {
        &*self.name()
    }

    fn field_slug(&self, executor: &Executor<'_, Context>) -> String {
        slugify(&*self.name())
    }
}

impl CategoryLevelFields for CategoryLevel {
    fn field_level(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Level, Walked>,
    ) -> Level {
        Level(self.level.clone())
    }

    fn field_category(
        &self,
        executor: &Executor<'_, Context>,
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
        let game = &executor.context().games()[self.category().game_id()];

        let runs: Vec<models::Run> = executor
            .context()
            .indicies()
            .runs_by_game_id_and_category_id_and_level_id()
            .get(&(
                *self.category().game_id(),
                *self.category().id(),
                Some(*self.level().id()),
            ))
            .map(Clone::clone)
            .unwrap_or_else(Default::default)
            .iter()
            .map(|run| models::Run::clone(run))
            .collect();

        let mut ranked = leaderboard(game, runs.iter(), include_obsolete);

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
        let game = &executor.context().games()[self.category().game_id()];

        let runs: Vec<models::Run> = executor
            .context()
            .indicies()
            .runs_by_game_id_and_category_id_and_level_id()
            .get(&(
                *self.category().game_id(),
                *self.category().id(),
                Some(*self.level().id()),
            ))
            .map(Clone::clone)
            .unwrap_or_else(Default::default)
            .iter()
            .map(|run| models::Run::clone(run))
            .collect();

        let progress = progression(game, runs.iter());

        progress.iter().map(|r| ProgressionRun(r.clone())).collect()
    }
}
