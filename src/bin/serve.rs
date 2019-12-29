#![allow(
    missing_docs,
    clippy::useless_attribute,
    clippy::useless_vec,
    unused_imports
)]
#![warn(missing_debug_implementations)]
#![deny(unconditional_recursion)]

use actix_cors::{self};
use actix_web::{self, web};
use async_std::{self};
use futures::{self};
use juniper::{
    self,
    http::{graphiql::graphiql_source, GraphQLRequest},
    FieldResult,
};
use speedruns::data::graphql;
use std::{convert::Infallible, sync::Arc};

#[allow(unused)] use log::{debug, error, info, trace, warn};

async fn graphiql() -> actix_web::HttpResponse {
    let html = juniper::http::graphiql::graphiql_source("/graphql");
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    schema: web::Data<Arc<graphql::Schema>>,
    query: web::Json<GraphQLRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let user = web::block(move || {
        let res = query.execute(&schema, &graphql::Context {});
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .header(actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Enable all debug logs by default.
    if std::env::var("RUST_LOG").unwrap_or_default().is_empty() {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    let schema = Arc::new(graphql::schema());

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(schema.clone())
            .wrap(actix_cors::Cors::new().finish())
            .wrap(actix_web::middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/{_:(graphiql)?}").route(web::get().to(graphiql)))
    });

    server.bind("127.0.0.1:8080")?.run().await
}
