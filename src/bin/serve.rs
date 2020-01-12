#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(missing_debug_implementations)]
#![deny(unconditional_recursion)]

use std::{fs::File, io::BufReader, sync::Arc};

use actix_cors::{self};
use actix_web::{self, web};

use juniper::{self, http::GraphQLRequest};
use lazy_static::lazy_static;
#[allow(unused)] use log::{debug, error, info, trace, warn};

use serde::de::DeserializeOwned;
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};

use speedruns::data::{
    database::{Database, Tables},
    graphql,
};

async fn graphiql() -> actix_web::HttpResponse {
    let html = juniper::http::graphiql::graphiql_source("/graphql");
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn playground() -> actix_web::HttpResponse {
    let html = juniper::http::playground::playground_source("/graphql");
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

lazy_static! {
    static ref DATABASE: Arc<Database> = {
        let tables: &'static Tables = Box::leak(Box::new(unpack_bundled_tables()));
        Database::new(tables).expect("database should be valid")
    };
}

async fn graphql(
    schema: web::Data<Arc<graphql::Schema>>,
    query: web::Json<GraphQLRequest>,
) -> actix_web::Result<actix_web::HttpResponse> {
    let database = DATABASE.clone();
    let user = web::block(move || {
        let res = query.execute(&schema, &graphql::Context { database });
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
            .service(web::resource("/").route(web::post().to(graphql)))
            .service(web::resource("/").route(web::get().to(graphql)))
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphql").route(web::get().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
    });

    server.bind("127.0.0.1:3001")?.run().await
}

fn unpack_bundled_tables() -> Tables {
    trace!("Unpacking bundled database...");

    let runs = read_table("data/normalized/runs.jsonl").expect("run data corrupt");
    let users = read_table("data/normalized/users.jsonl").expect("user data corrupt");
    let games = read_table("data/normalized/games.jsonl").expect("game data corrupt");
    let categories =
        read_table("data/normalized/categories.jsonl").expect("category data corrupt");
    let levels = read_table("data/normalized/levels.jsonl").expect("level data corrupt");

    Tables::new(runs, users, games, categories, levels)
}

pub fn read_table<T: DeserializeOwned>(
    path: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let buffer = BufReader::new(&file);
    let deserializer = JsonDeserializer::from_reader(buffer);
    let json_results = deserializer.into_iter::<JsonValue>();
    Ok(json_results
        .map(Result::unwrap)
        .map(T::deserialize)
        .map(Result::unwrap)
        .collect())
}
