use juniper::{FieldResult, RootNode};

#[allow(unused)]
use juniper::{
    graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value,
    object, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue,
    ScalarValue,
};

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
    pub fn user(
        context: &Context,
        id: Option<String>,
        name: Option<String>,
    ) -> FieldResult<User> {
        Ok(User {
            id:   id.unwrap_or_else(|| "123".to_string()),
            name: name.unwrap_or_else(|| "Nom".to_string()),
        })
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
