use std::sync::Arc;

use futures::future;
use hyper::{
    header::HeaderValue,
    rt::{self, Future},
    service::service_fn,
    Body, Method, Request, Response, StatusCode,
};
#[allow(unused)] use log::{debug, error, info, trace, warn};

use crate::{data::leaderboard::rank_runs, Database};

mod views;

use views::*;

pub type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

/// A web server displaying a static snapshot of https://speedrun.com leaderboards.
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

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/icon.gif") => {
            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("image/gif"));
            *response.body_mut() = Body::from(include_bytes!("static/srca.gif").as_ref());
        }

        (&Method::GET, "/style.css") => {
            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("text/css"));
            *response.body_mut() = Body::from(include_bytes!("static/style.css").as_ref());
        }

        (&Method::GET, "/") => {
            Homepage.html_to(&mut response);
        }

        (&Method::GET, "/celeste/anypercent") => {
            let game_slug = "celeste";
            let category_slug = "anypercent";

            let user = database
                .clone()
                .user_by_slugify("TGH")
                .expect("TGH in database");

            let celeste = database
                .clone()
                .game_by_slugify("celeste")
                .expect("Celeste in database");
            let any_percent = celeste
                .category_by_slugify("Any%")
                .expect("Any% in Celeste");
            let runs = any_percent.full_runs();

            let clear = celeste
                .category_by_slugify("Clear")
                .expect("Any% in Celeste");
            let forsaken_city = celeste
                .level_by_slugify("Forsaken City")
                .expect("Forsaken City in Celeste");
            let runs = clear.level_runs(&forsaken_city);

            let leaderboards = rank_runs(database.clone(), &runs);

            Debug(&leaderboards).html_to(&mut response);

            // let games = game

            // let game = database.games_by_slug()[game_slug];;
            // let runs = &database.runs_by_game_id()[game.id()];
            // let category = database
            //     .categories()
            //     .values()
            //     .find(|c| c.game_id() == game.id() && c.name() == "Any%")
            //     .unwrap();
            // let runs = runs
            //     .iter()
            //     .filter(|r| r.category_id() == category.id())
            //     .cloned()
            //     .collect::<Vec<_>>();
            // let ranks = database.rank_runs(&runs);

            // let view = LeaderboardPage {
            //     game,
            //     category,
            //     level: None,
            //     ranks,
            // };

            // response
            //     .headers_mut()
            //     .insert("Content-Type", HeaderValue::from_static("text/html"));

            // let render = view.render().into_string();
            // *response.body_mut() = Body::from(render);
        }

        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Box::new(future::ok(response))
}
