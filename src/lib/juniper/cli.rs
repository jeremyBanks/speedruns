#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec)]
#![warn(
    missing_debug_implementations,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

use async_std::sync::RwLock;
use std::{fs::File, io::BufReader, sync::Arc};

use actix_cors::{self};
use actix_web::{self, middleware, web, HttpResponse};

use juniper::{self, http::GraphQLRequest};
use lazy_static::lazy_static;

use log::{error, info, warn};
use serde::de::DeserializeOwned;
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};

use speedruns_database::{Database, Tables};

async fn graphiql() -> HttpResponse {
    let html = juniper::http::graphiql::graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn playground() -> HttpResponse {
    let html = juniper::http::playground::playground_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    schema: web::Data<Arc<crate::Schema>>,
    query: web::Json<GraphQLRequest>,
) -> actix_web::Result<HttpResponse> {
    let lock = DATABASE.read().await;
    let database: Arc<Database> = lock.clone().unwrap();

    let user = web::block(move || {
        let res = query.execute(&schema, &crate::Context { database });
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .header(actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(user))
}

#[cfg(target_os = "linux")]
async fn diediedie() -> HttpResponse {
    unsafe {
        use libc::{getppid, kill, SIGKILL};
        kill(getppid(), SIGKILL);
    }

    panic!("/diediedie")
}

#[cfg(not(target_os = "linux"))]
async fn diediedie() -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body("/diediedie only works on linux")
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Serves imported data from a GraphQL server. All data is loaded into memory, not served
/// from disk.
#[argh(subcommand, name = "serve")]
pub struct Args {
    /// port to run server on
    #[argh(option)]
    port: Option<u32>,
    /// whether to skip the database import (such as if you only need to run the server to
    /// briefly download the schema)
    #[argh(switch)]
    no_data: bool,
}

lazy_static! {
    static ref DATABASE: RwLock<Option<Arc<Database>>> = RwLock::new(None);
}

pub async fn main(args: Args) -> std::io::Result<()> {
    info!("Initializing server.");
    *(DATABASE.write().await) = Some(Arc::new(
        Database::try_new(Arc::new(unpack_tables(args.no_data)))
            .expect("database should be valid"),
    ));

    info!("Initializing schema.");
    let schema = Arc::new(crate::schema());

    info!("Initializing server.");
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(middleware::Compress::default())
            .data(schema.clone())
            .wrap(actix_cors::Cors::new().finish())
            .wrap(actix_web::middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
            .service(web::resource("/diediedie").route(web::get().to(diediedie)))
    });

    info!("Binding server.");
    server
        .bind(format!("127.0.0.1:{}", args.port.unwrap_or(3001)))?
        .run()
        .await
}

fn unpack_tables(no_data: bool) -> Tables {
    if no_data {
        info!("Skipping database import, will run with no data!");
        return Tables::new(vec![], vec![], vec![], vec![], vec![]);
    }

    info!("Unpacking database...");

    let mut runs = read_table("data/imported/runs.jsonl").expect("run data corrupt");
    info!("{} runs.", runs.len());
    let supplemental =
        read_table("data/supplemental/runs.jsonl").expect("supplemental run data corrupt");
    info!("{} supplemental runs.", supplemental.len());
    let users = read_table("data/imported/users.jsonl").expect("user data corrupt");
    info!("{} users.", users.len());
    let games = read_table("data/imported/games.jsonl").expect("game data corrupt");
    info!("{} games.", games.len());
    let categories =
        read_table("data/imported/categories.jsonl").expect("category data corrupt");
    info!("{} categories.", categories.len());
    let levels = read_table("data/imported/levels.jsonl").expect("level data corrupt");
    info!("{} levels.", levels.len());

    runs.extend(supplemental.into_iter());

    Tables::new(games, categories, levels, runs, users)
}

pub fn read_table<T: DeserializeOwned>(
    path: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    Ok(match File::open(path) {
        Ok(file) => {
            let buffer = BufReader::new(&file);
            let deserializer = JsonDeserializer::from_reader(buffer);
            let json_results = deserializer.into_iter::<JsonValue>();
            json_results
                .map(Result::unwrap)
                .map(T::deserialize)
                .map(Result::unwrap)
                .collect()
        }
        Err(err) => {
            error!("Failed to load table: {:?}", err);
            vec![]
        }
    })
}
