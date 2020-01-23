juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        parent: Parent @juniper(ownership: "owned", infallible: true)
    }

    type Parent {
        child: Child! @juniper(ownership: "owned", infallible: true)
    }

    type Child {
        id: ID! @juniper(ownership: "owned", infallible: true)
    }
}

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;
pub struct Parent;
pub struct Child;

impl QueryFields for Query {
    fn field_parent(
        &self,
        _executor: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Parent, Walked>,
    ) -> Option<Parent> {
        Some(Parent)
    }
}

impl ParentFields for Parent {
    fn field_child(
        &self,
        _executor: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Child, Walked>,
    ) -> Child {
        Child
    }
}

impl ChildFields for Child {
    fn field_id(&self, _executor: &juniper::Executor<'_, Context>) -> juniper::ID {
        "example-id".to_string().into()
    }
}

fn main() {
    let schema = Schema::new(Query {}, juniper::EmptyMutation::new());

    let query = r#"
        fragment ChildFragment on Child {
            id
        }

        fragment ParentFragment on Parent {
            child {
                ...ChildFragment
            }
        }

        query {
            parent {
                ...ParentFragment
            }
        }
    "#;

    juniper::execute(query, None, &schema, &juniper::Variables::new(), &Context).unwrap();
}
