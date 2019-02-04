#![feature(proc_macro_hygiene)]
#![warn(missing_debug_implementations, missing_docs)]
#![allow(unused_imports, missing_debug_implementations, missing_docs)]
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    error::Error,
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader, BufWriter, Read},
    num::NonZeroU64 as p64,
    ops::Deref,
    rc::Rc,
};

use futures::future;
use hyper::{
    header::HeaderValue,
    rt::{self, Future, Stream},
    service::{service_fn, service_fn_ok},
    Body, Method, Request, Response, Server, StatusCode,
};
use lazy_static::lazy_static;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use maud::{html, Markup, Render};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::Validate;
use xz2::read::XzDecoder;

use speedruncom_data_tools::{
    database::Database, escape_html::Escape, normalized_types::*, BoxErr,
};

mod views;
use views::*;

pub type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

lazy_static! {
    pub static ref DATABASE: Database = unpack_bundled_database();
    pub static ref GAMES_BY_SLUG: HashMap<&'static str, &'static Game> =
        DATABASE.games_by_slug();
    pub static ref RUNS_BY_GAME_ID: HashMap<p64, Vec<&'static Run>> =
        DATABASE.runs_by_game_id();
}

pub fn main() -> Result<(), BoxErr> {
    env_logger::try_init_from_env(env_logger::Env::new().default_filter_or(format!(
        "{}=trace,speedruncom_data_tools=trace,hyper=debug",
        module_path!()
    )))?;

    let server =
        Server::bind(&([127, 0, 0, 1], 59330).into()).serve(|| service_fn(respond));
    let addr = server.local_addr();

    let url = format!("http://{}", addr);
    info!("Listening at {}", &url);
    // webbrowser::open(&url)?;

    rt::run(server.map_err(|e| error!("server error: {}", e)));

    Ok(())
}

fn respond(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let game = GAMES_BY_SLUG["Celeste"];
            let runs = &RUNS_BY_GAME_ID[game.id()];
            let category = DATABASE
                .categories()
                .values()
                .filter(|c| c.game_id() == game.id() && c.name() == "Any%")
                .next()
                .unwrap();
            let runs = runs
                .iter()
                .filter(|r| r.category_id() == category.id())
                .cloned()
                .collect::<Vec<_>>();
            let ranks = DATABASE.rank_runs(&runs);

            let view = LeaderboardPage {
                game,
                category,
                level: None,
                ranks,
            };

            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("text/html"));

            let render = view.render().into_string();
            *response.body_mut() = Body::from(render);
        }

        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }
    Box::new(future::ok(response))
}

fn load_data<T: DeserializeOwned>(
    reader: &mut &[u8],
    database: &mut Database,
    loader: impl Fn(&mut Database, T),
) -> Result<(), BoxErr> {
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
        loader(database, item?);
    }
    Ok(())
}

fn unpack_bundled_database() -> Database {
    let mut database = Database::new();

    trace!("Unpacking bundled database...");

    load_data(
        &mut include_bytes!("../../data/normalized/categories.bin.xz").as_ref(),
        &mut database,
        Database::insert_category,
    )
    .expect("category data corrupt");
    load_data(
        &mut include_bytes!("../../data/normalized/games.bin.xz").as_ref(),
        &mut database,
        Database::insert_game,
    )
    .expect("game data corrupt");
    load_data(
        &mut include_bytes!("../../data/normalized/levels.bin.xz").as_ref(),
        &mut database,
        Database::insert_level,
    )
    .expect("level data corrupt");
    load_data(
        &mut include_bytes!("../../data/normalized/runs.bin.xz").as_ref(),
        &mut database,
        Database::insert_run,
    )
    .expect("run data corrupt");
    load_data(
        &mut include_bytes!("../../data/normalized/users.bin.xz").as_ref(),
        &mut database,
        Database::insert_user,
    )
    .expect("user data corrupt");

    database.validate().expect("database state invalid");

    database
}
