use std::sync::Arc;

use futures::future;
use hyper::{
    header::HeaderValue,
    rt::{self, Future},
    service::service_fn,
    Body, Method, Request, Response, StatusCode,
};
#[allow(unused)] use log::{debug, error, info, trace, warn};

use crate::{data::leaderboard::rank_runs, server::path::Path, Database};

pub(self) mod path;
pub(self) mod views;
pub(self) use views::*;

/// A Boxed Future for interfacing with Hyper.
pub type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

/// A web server displaying a static snapshot of https://speedrun.com leaderboards.
#[derive(Debug)]
pub struct Server {
    database: Arc<Database>,
}

impl Server {
    pub fn new(database: Arc<Database>) -> Self {
        Server { database }
    }

    pub fn run(&mut self) {
        // Automatically selectiing different ports would be bad for production, but is
        // convenient for dev.
        let addresses = vec![
            ([0, 0, 0, 0], 80),
            ([127, 0, 0, 1], 59330),
            ([127, 0, 0, 1], 59331),
            ([127, 0, 0, 1], 59332),
            ([127, 0, 0, 1], 59333),
            ([127, 0, 0, 1], 0),
        ];

        let mut binding = None;
        for address in addresses {
            let address = address.into();
            match hyper::Server::try_bind(&address) {
                Ok(binding_) => {
                    binding = Some(binding_);
                    break
                }
                Err(error) => {
                    warn!("Failed to bind {:?}: {:?}", &address, &error);
                }
            }
        }

        let database = self.database.clone();
        let server = binding.expect("failed to bind any port").serve(move || {
            let database = database.clone();
            service_fn(move |req| respond(req, database.clone()))
        });
        let addr = server.local_addr();

        let url = format!("http://{}", addr);
        info!("Listening at {}", &url);

        println!("Listening at {}", &url);

        rt::run(server.map_err(|e| error!("server error: {}", e)));
    }
}

fn respond(req: Request<Body>, database: Arc<Database>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    let mut path = req.uri().path().to_string();
    let json_view = path.ends_with(".json");
    if json_view {
        path.truncate(path.len() - ".json".len());
    }

    match (req.method(), path.as_ref()) {
        (&Method::GET, "/style.css") => {
            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("text/css"));
            *response.body_mut() = Body::from(include_bytes!("static/style.css").as_ref());
        }
        (&Method::GET, "/srca.gif") => {
            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("image/gif"));
            *response.body_mut() =
                Body::from(include_bytes!("static/srca-icon.gif").as_ref());
        }

        (&Method::GET, "/src.png") => {
            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("image/png"));
            *response.body_mut() =
                Body::from(include_bytes!("static/src-icon.png").as_ref());
        }

        (&Method::GET, path) => match Path::from_str(path, database.clone()).unwrap() {
            Path::Home => Homepage.write_response(&mut response, json_view),
            Path::FullCategory(category) => {
                let runs = category.full_runs();
                let ranks = rank_runs(database.clone(), &runs);
                LeaderboardPage {
                    game: category.game().clone(),
                    category: category.clone(),
                    level: None,
                    ranks,
                }
                .write_response(&mut response, json_view);
            }
            Path::LevelCategory(category, level) => {
                let runs = category.level_runs(&level);
                let ranks = rank_runs(database.clone(), &runs);
                LeaderboardPage {
                    game: category.game().clone(),
                    category: category.clone(),
                    level: Some(level.clone()),
                    ranks,
                }
                .write_response(&mut response, json_view);
            }
            _ => unimplemented!(),
        },

        _ => {
            *response.status_mut() = StatusCode::BAD_REQUEST;
            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("image/plain"));
            *response.body_mut() = Body::from("bad request");
        }
    }

    Box::new(future::ok(response))
}
