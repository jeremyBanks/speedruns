#[allow(unused)]
use juniper::{
    graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value,
    object, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue,
    ScalarValue,
};
use juniper::{Executor, ID};
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/schema.graphql");

pub fn schema() -> Schema {
    Schema::new(Speedruns {}, Speedruns {})
}

#[derive(Debug, Clone)]
pub struct Context;

impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Speedruns;

#[derive(Debug, Clone)]
pub struct Game;

#[derive(Debug, Clone)]
pub struct Run;

#[derive(Debug, Clone)]
pub struct LeaderboardRun;

#[derive(Debug, Clone)]
pub struct Category;

impl SpeedrunsFields for Speedruns {
    fn field_game(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Game, Walked>,
        _slug: String,
    ) -> Option<Game> {
        Some(Game)
    }
}

impl GameFields for Game {
    fn field_game_categories(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Category, Walked>,
    ) -> Vec<Category> {
        vec![Category]
    }
}

impl RunFields for Run {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        "example-id".to_string().into()
    }
}

impl LeaderboardRunFields for LeaderboardRun {
    fn field_run(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Run, Walked>,
    ) -> Run {
        Run
    }
}

impl CategoryFields for Category {
    fn field_leaderboard(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, LeaderboardRun, Walked>,
        _level_slug: Option<String>,
        _include_obsolete: bool,
    ) -> Vec<LeaderboardRun> {
        vec![LeaderboardRun]
    }
}

fn main() {
    let schema = schema();

    let query = r#"
        fragment GameRun on Run {
            id
        }
        fragment GameLeaderboardRun on LeaderboardRun {
            run {
                ...GameRun
            }
        }
        query GetGamePage {
            game: game(slug: "wc2") {
                gameCategories {
                    leaderboard {
                        ...GameLeaderboardRun
                    }
                }
            }
        }
    "#;

    juniper::execute(
        query,
        None,
        &schema,
        &juniper::Variables::new(),
        &Context,
    )
    .unwrap();
}
