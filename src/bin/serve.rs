#![feature(try_blocks)]
#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(
    missing_debug_implementations,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
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
        let tables: &'static Tables = Box::leak(Box::new(unpack_tables()));
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

async fn diediedie() -> actix_web::HttpResponse {
    unsafe {
        use libc::{getppid, kill, SIGKILL};
        kill(getppid(), SIGKILL);
    }

    panic!("/diediedie")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Enable all debug logs by default.
    if std::env::var("RUST_LOG").unwrap_or_default().is_empty() {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    info!("Initializing server.");
    lazy_static::initialize(&DATABASE);

    info!("Initializing schema.");
    let schema = Arc::new(graphql::schema());

    info!("Initializing server.");
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(schema.clone())
            .wrap(actix_cors::Cors::new().finish())
            .wrap(actix_web::middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
            .service(web::resource("/diediedie").route(web::get().to(diediedie)))
    });

    info!("Binding server.");
    server.bind("127.0.0.1:3001")?.run().await
}

fn unpack_tables() -> Tables {
    info!("Unpacking database...");

    let mut runs = read_table("data/normalized/runs.jsonl").expect("run data corrupt");
    info!("{} runs.", runs.len());
    let supplemental =
        read_table("data/supplemental/runs.jsonl").expect("supplemental run data corrupt");
    info!("{} supplemental runs.", supplemental.len());
    let users = read_table("data/normalized/users.jsonl").expect("user data corrupt");
    info!("{} users.", users.len());
    let games = read_table("data/normalized/games.jsonl").expect("game data corrupt");
    info!("{} games.", games.len());
    let categories =
        read_table("data/normalized/categories.jsonl").expect("category data corrupt");
    info!("{} categories.", categories.len());
    let levels = read_table("data/normalized/levels.jsonl").expect("level data corrupt");
    info!("{} levels.", levels.len());

    runs.extend(supplemental.into_iter());

    Tables::new(runs, users, games, categories, levels)
}

pub fn read_table<T: DeserializeOwned>(
    path: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let result: Result<Vec<T>, Box<dyn std::error::Error>> = try {
        let file = File::open(path)?;
        let buffer = BufReader::new(&file);
        let deserializer = JsonDeserializer::from_reader(buffer);
        let json_results = deserializer.into_iter::<JsonValue>();
        json_results
            .map(Result::unwrap)
            .map(T::deserialize)
            .map(Result::unwrap)
            .collect()
    };
    match result {
        Ok(result) => Ok(result),
        Err(err) => {
            error!("Failed to load table: {:?}", err);
            Ok(vec![])
        }
    }
}
