#![allow(
    missing_docs,
    clippy::useless_attribute,
    clippy::useless_vec,
    unused_imports
)]
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

    pub fn new() -> Schema {
        Schema::new(Query {}, Mutation {})
    }
}

async fn graphiql() -> actix_web::HttpResponse {
    let html = juniper::http::graphiql::graphiql_source("/graphql");
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    schema: web::Data<Arc<schema::Schema>>,
    query: web::Json<GraphQLRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let user = web::block(move || {
        let res = query.execute(&schema, &schema::Context {});
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Enable all debug logs by default.
    if std::env::var("RUST_LOG").unwrap_or_default().is_empty() {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    let schema = Arc::new(schema::new());

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(schema.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/{_:(graphiql)?}").route(web::get().to(graphiql)))
    });

    server.bind("127.0.0.1:8080")?.run().await
}
