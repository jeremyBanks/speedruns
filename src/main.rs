#[allow(unused)]
use juniper::{
    graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value, object,
    GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue, ScalarValue,
};
use juniper::{Executor, ID};
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/schema.graphql");

pub fn schema() -> Schema {
    Schema::new(Query {}, Query {})
}

#[derive(Debug, Clone)]
pub struct Context;

impl juniper::Context for Context {}

#[derive(Debug)]
pub struct Query;

#[derive(Debug, Clone)]
pub struct Grandparent;

#[derive(Debug, Clone)]
pub struct Grandchild;

#[derive(Debug, Clone)]
pub struct Child;

#[derive(Debug, Clone)]
pub struct Parent;

impl QueryFields for Query {
    fn field_grandparent(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Grandparent, Walked>,
    ) -> Option<Grandparent> {
        Some(Grandparent)
    }
}

impl GrandparentFields for Grandparent {
    fn field_parent(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Parent, Walked>,
    ) -> Vec<Parent> {
        vec![Parent]
    }
}

impl ParentFields for Parent {
    fn field_child(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Child, Walked>,
    ) -> Vec<Child> {
        vec![Child]
    }
}

impl ChildFields for Child {
    fn field_grandchild(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Grandchild, Walked>,
    ) -> Grandchild {
        Grandchild
    }
}

impl GrandchildFields for Grandchild {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> ID {
        "example-id".to_string().into()
    }
}

fn main() {
    let schema = schema();

    let query = r#"
        fragment GrandchildFragment on Grandchild {
            id
        }

        fragment ChildFragment on Child {
            grandchild {
                ...GrandchildFragment
            }
        }
        query {
            grandparent {
                parent {
                    child {
                        ...ChildFragment
                    }
                }
            }
        }
    "#;

    juniper::execute(query, None, &schema, &juniper::Variables::new(), &Context).unwrap();
}
