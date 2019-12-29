#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(missing_debug_implementations)]
#![deny(unconditional_recursion)]

use actix_web::{self, web};
use async_std::{self};
use futures::{self};
use juniper::{
    self,
    http::{graphiql::graphiql_source, GraphQLRequest},
    FieldResult,
};
use std::{convert::Infallible, sync::Arc};

#[allow(unused)] use log::{debug, error, info, trace, warn};

pub mod schema {
    use juniper::{FieldResult, RootNode};
    // juniper macro components
    #[allow(unused)]
    use juniper::{
        graphql_interface, graphql_object, graphql_scalar, graphql_union, graphql_value,
        object, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalarValue,
        ScalarValue,
    };

    #[derive(Debug, Clone)]
    pub struct Context {}
    impl juniper::Context for Context {}

    #[derive(GraphQLEnum)]
    enum Episode {
        NewHope,
        Empire,
        Jedi,
    }

    #[derive(GraphQLObject)]
    #[graphql(description = "A humanoid creature in the Star Wars universe")]
    struct Human {
        id:          String,
        name:        String,
        appears_in:  Vec<Episode>,
        home_planet: String,
    }

    #[derive(GraphQLInputObject)]
    #[graphql(description = "A humanoid creature in the Star Wars universe")]
    struct NewHuman {
        name:        String,
        appears_in:  Vec<Episode>,
        home_planet: String,
    }

    #[derive(Debug, Default)]
    pub struct Query {}

    #[juniper::object(Context = Context)]
    impl Query {
        pub fn human(context: &Context, id: String) -> FieldResult<Human> {
            Ok(Human {
                id:          "1234".to_owned(),
                name:        "Luke".to_owned(),
                appears_in:  vec![Episode::NewHope],
                home_planet: "Mars".to_owned(),
            })
        }
    }

    #[derive(Debug, Default)]
    pub struct Mutation {}

    #[juniper::object(Context = Context)]
    impl Mutation {}

    pub type Schema = RootNode<'static, Query, Mutation>;

    pub fn new() -> Schema {
        Schema::new(Query {}, Mutation {})
    }
}

async fn graphiql() -> actix_web::HttpResponse {
    let html = graphiql_source("/graphql");
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<schema::Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &schema::Context {});
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").unwrap_or_default().is_empty() {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(schema::new());

    // Start http server
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(schema.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
