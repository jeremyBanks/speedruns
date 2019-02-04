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
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::Validate;
use xz2::read::XzDecoder;

use speedruncom_data_tools::{
    database::Database, escape_html::Escape, normalized_types::*, BoxErr,
};

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

    let server = Server::bind(&([127, 0, 0, 1], 0).into()).serve(|| service_fn(respond));
    let addr = server.local_addr();

    let url = format!("http://{}", addr);
    info!("Listening at {}", &url);
    webbrowser::open(&url)?;

    rt::run(server.map_err(|e| error!("server error: {}", e)));

    Ok(())
}

fn respond(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let celeste = GAMES_BY_SLUG["Celeste"];
            let runs = &RUNS_BY_GAME_ID[celeste.id()];
            let any_percent_id = DATABASE
                .categories()
                .values()
                .filter(|c| c.game_id() == celeste.id() && c.name() == "Any%")
                .next()
                .unwrap()
                .id();
            let any_percent_runs = runs
                .iter()
                .filter(|r| r.category_id() == any_percent_id)
                .cloned()
                .collect::<Vec<_>>();
            let any_percent_leaderboard = DATABASE.rank_runs(&any_percent_runs);

            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("text/html"));
            let mut body: Vec<u8> = Vec::new();

            writeln!(&mut body, "<!doctype html><title>speedruns</title><body>").unwrap();
            writeln!(
                &mut body,
                "{}",
                "<style>pre { white-space: pre-wrap; }</style>"
            )
            .unwrap();

            for run in any_percent_leaderboard {
                writeln!(&mut body, "<pre>{}</pre>", Escape(&format!("{:#?}", &run)))
                    .unwrap();
            }

            *response.body_mut() = Body::from(body);
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
