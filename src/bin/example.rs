#![feature(arbitrary_self_types, assoc_int_consts)]
#![allow(unused)]
use async_std::{
    prelude::*,
    sync::{channel, Arc, RwLock},
    task::{sleep, spawn},
};
use log::{debug, error, info, trace, warn};
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, HashSet},
    io::Write,
    rc::Rc,
    time::{Duration, Instant},
};

#[derive(Debug)]
enum Request {
    GetNames,
    AddName(String),
    RemoveName(String),
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
        match request {
            Request::GetNames => {}
            Request::AddName(name) => {
                self.database().add_name(name).await;
            }
            Request::RemoveName(name) => {
                self.database().remove_name(name).await;
            }
        }

        let names = self.database().get_names().await;

        Response { names }
    }
}

#[derive(Debug)]
struct Database {
    names: HashSet<String>,
}

impl Database {
    pub async fn get_names(&self) -> Vec<String> {
        sleep(Duration::from_secs_f64(0.25)).await;
        self.names.iter().cloned().collect()
    }

    pub async fn add_name(&mut self, name: String) {
        sleep(Duration::from_secs_f64(2.25)).await;
        self.names.insert(name);
    }

    pub async fn remove_name(&mut self, name: String) {
        sleep(Duration::from_secs_f64(4.5)).await;
        self.names.remove(&name);
    }
}

#[async_std::main]
async fn main() {
    init_logger();

    let app = Arc::new(WebApp::new());

    let (sender, receiver) = channel(32);

    let client = spawn(async move {
        loop {
            sleep(Duration::from_secs_f64(1.0)).await;

            let request = Request::GetNames;
            sender.send(request).await;
        }
    });

    let server = spawn(async move {
        loop {
            let request = receiver.recv().await.unwrap();
            debug!(" request: {:?}", request);

            let app_for_task = app.clone();
            spawn(async {
                let response = app_for_task.handle(request).await;
                debug!("response: {:?}", response);
            });
        }
    });

    client.join(server).await;
}

fn init_logger() {
    // Configure logger to display relative instead of absolute time.
    let start = Instant::now();
    env_logger::Builder::new()
        .parse_filters("example=debug")
        .format(move |buffer, record| {
            let elapsed = (Instant::now() - start).as_secs_f64(); 
            writeln!(buffer, "[t={:>6.1}s] {}", elapsed, record.args())
        })
        .init();
}
