#![allow(missing_docs, clippy::useless_attribute, unused_imports)]
#![warn(missing_debug_implementations)]
#![deny(unconditional_recursion)]

use std::{convert::Infallible, sync::Arc};

use actix_cors::{self};
use actix_web::{self, web};
use async_std::{self};
use futures::{self};
use juniper::{
    self,
    http::{graphiql::graphiql_source, GraphQLRequest},
    FieldResult,
};
use lazy_static::lazy_static;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use serde::de::DeserializeOwned;
use xz2::read::XzDecoder;

use speedruns::data::{
    database::{Database, Tables},
    graphql,
};

async fn graphiql() -> actix_web::HttpResponse {
    let html = juniper::http::graphiql::graphiql_source("/");
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn playground() -> actix_web::HttpResponse {
    let html = juniper::http::playground::playground_source("/");
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
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
    });

    server.bind("127.0.0.1:3001")?.run().await
}

fn unpack_bundled_tables() -> Tables {
    trace!("Unpacking bundled database...");

    let runs = unpack_table(
        &mut include_bytes!(concat!(env!("OUT_DIR"), "/data/normalized/runs.bin.xz"))
            .as_ref(),
    )
    .expect("run data corrupt");

    let users = unpack_table(
        &mut include_bytes!(concat!(env!("OUT_DIR"), "/data/normalized/users.bin.xz"))
            .as_ref(),
    )
    .expect("user data corrupt");

    let games = unpack_table(
        &mut include_bytes!(concat!(env!("OUT_DIR"), "/data/normalized/games.bin.xz"))
            .as_ref(),
    )
    .expect("game data corrupt");

    let categories = unpack_table(
        &mut include_bytes!(concat!(
            env!("OUT_DIR"),
            "/data/normalized/categories.bin.xz"
        ))
        .as_ref(),
    )
    .expect("category data corrupt");

    let levels = unpack_table(
        &mut include_bytes!(concat!(env!("OUT_DIR"), "/data/normalized/levels.bin.xz"))
            .as_ref(),
    )
    .expect("level data corrupt");

    Tables::new(runs, users, games, categories, levels)
}

fn unpack_table<T: DeserializeOwned>(
    reader: &mut &[u8],
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let mut items = vec![];
    let mut decompressor = XzDecoder::new(reader);
    loop {
        // We have left no way to detect the last item except for EOF.
        let item = bincode::deserialize_from::<_, T>(&mut decompressor);
        if let Err(ref error) = item {
            if let bincode::ErrorKind::Io(ref error) = **error {
                trace!("Assuming IO error is end of data EOF: {:?}", error);
                break
            }
        }
        items.push(item?);
    }
    Ok(items)
}
