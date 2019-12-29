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

#[derive(Debug, GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A user")]
pub struct User {
    id:   String,
    name: String,
}

#[derive(Debug, Default)]
pub struct Query {}

#[juniper::object(Context = Context)]
impl Query {
    /// Get a user by id or slug. throws an error if none are specified, or no user matches
    /// all that are specified.
    pub fn user(
        context: &Context,
        id: Option<String>,
        slug: Option<String>,
    ) -> FieldResult<User> {
        Err(FieldError::from("not implemented"))
    }
}

#[derive(Debug, Default)]
pub struct Mutation {}

#[juniper::object(Context = Context)]
impl Mutation {
    // workaround for https://git.io/JeNXr
    pub fn dummy(context: &Context) -> FieldResult<f64> {
        Ok(0.0)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
