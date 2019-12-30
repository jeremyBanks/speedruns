use juniper::{FieldError, FieldResult, RootNode};

#[allow(unused)]
use juniper::{
    graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value,
    object, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue,
    ScalarValue,
};

/// Request context -- nothing, but maybe we'll add one later.
#[derive(Debug)]
pub struct Context {}
impl juniper::Context for Context {}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A user of speedrun.com.")]
pub struct User {
    id:   String,
    slug: String,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A game on speedrun.com.")]
pub struct Game {
    id:   String,
    slug: String,
    name: String,
}

#[derive(Debug, Default)]
pub struct Query {}

#[juniper::object(Context = Context)]
impl Query {
    #[graphql(description = "
        Get a user by id or slug.
    ")]
    pub fn user(
        context: &Context,
        id: Option<String>,
        slug: Option<String>,
    ) -> FieldResult<User> {
        Err(FieldError::from("not implemented"))
    }

    #[graphql(description = "
        Get a game by id or slug.
    ")]
    pub fn game(
        context: &Context,
        id: Option<String>,
        slug: Option<String>,
    ) -> FieldResult<Game> {
        Err(FieldError::from("not implemented"))
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
