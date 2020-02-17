#![feature(arbitrary_self_types)]
#![allow(unused)]
use async_std::prelude::*;
use log::{debug, error, info, trace, warn};
use rand::prelude::*;
use std::{
    collections::{BTreeMap, HashSet},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

#[derive(Debug)]
struct Request {
    names_to_add:    Vec<String>,
    names_to_remove: Vec<String>,
}

#[derive(Debug)]
struct Response {
    names: Vec<String>,
}

#[derive(Debug)]
struct WebApp {
    database: Database,
}

impl WebApp {
    fn new() -> WebApp {
        WebApp {
            database: Database {
                names: vec!["Seed".to_string()].into_iter().collect(),
            },
        }
    }

    fn database(&self) -> &Database {
        &self.database
    }

    async fn handle(self: Arc<Self>, request: Request) -> Response {
        // Pretend we're waiting for a slow backend.
        async_std::task::sleep(Duration::from_secs(5)).await;
        Response {
            names: self.database().names().cloned().collect(),
        }
    }
}

#[derive(Debug)]
struct Database {
    names: HashSet<String>,
}

impl Database {
    fn names(&self) -> impl Iterator<Item = &String> {
        self.names.iter()
    }
}

#[async_std::main]
async fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let app = Arc::new(WebApp::new());

    loop {
        // For some variety, let's say that every one second...
        async_std::task::sleep(Duration::from_secs(1)).await;
        // ...we have a 20% probability...
        if 0.20 > rand::thread_rng().gen_range(0.0, 1.0) {
            // ...of simulating a request.
            let request = Request {
                names_to_add:    vec![],
                names_to_remove: vec![],
            };
            debug!(" request: {:?}", request);

            let app_for_task = app.clone();
            async_std::task::spawn(async {
                let response = app_for_task.handle(request).await;
                debug!("response: {:?}", response);
            });
        }
    }
}
