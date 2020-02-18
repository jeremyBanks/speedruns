#![feature(arbitrary_self_types, assoc_int_consts)]
#![allow(unused_imports)]
use async_std::{
    prelude::*,
    sync::{channel, Arc, RwLock},
    task::{sleep, spawn},
};
use log::{debug, error, info, trace, warn};
use std::{
    collections::{BTreeMap, HashSet},
    io::Write,
    rc::Rc,
    time::{Duration, Instant},
};
use rand::prelude::*;

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
                names: RwLock::new(Default::default()),
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
    names: RwLock<HashSet<String>>,
}

impl Database {
    pub async fn get_names(&self) -> Vec<String> {
        debug!("database: get_names() requesting read lock");
        let names = self.names.read().await;
        debug!("database: get_names() got read lock");
        sleep(Duration::from_secs_f64(0.25)).await;
        names.iter().cloned().collect()
    }

    pub async fn add_name(&self, name: String) {
        debug!("database: add_name({:?}) requesting write lock", name);
        let mut names = self.names.write().await;
        debug!("database: add_name({:?}) got write lock", name);
        sleep(Duration::from_secs_f64(2.25)).await;
        names.insert(name);
    }

    pub async fn remove_name(&self, name: String) {
        debug!("database: remove_name({:?}) requesting write lock", name);
        let mut names = self.names.write().await;
        debug!("database: remove_name({:?}) got write lock", name);
        sleep(Duration::from_secs_f64(4.5)).await;
        names.remove(&name);
    }
}

#[async_std::main]
async fn main() {
    init_logger();

    let app = Arc::new(WebApp::new());

    let (sender, receiver) = channel(32);

    let client = spawn(async move {
        sender.send(Request::AddName("Jeremy".to_string())).await;
        sleep(Duration::from_secs_f64(0.0125)).await;
        sender.send(Request::AddName("Banks".to_string())).await;
        sleep(Duration::from_secs_f64(0.0125)).await;
        sender.send(Request::RemoveName("Banks".to_string())).await;

        loop {
            sleep(Duration::from_secs_f64(1.0)).await;

            let request = Request::GetNames;
            sender.send(request).await;
        }
    });

    let server = spawn(async move {
        loop {
            let request = receiver.recv().await.unwrap();
            debug!("server: got request {:?}", request);

            let app_for_task = app.clone();
            spawn(async {
                let request_s = format!("{:?}", request);

                let response = app_for_task.handle(request).await;
                debug!("server: sent response {:?} for request {}", response, request_s);
            });
        }
    });

    client.join(server).await;
}

fn init_logger() {
    // Configure a logger displaying elapsed time since now.
    let start = Instant::now();
    env_logger::Builder::new()
        .parse_filters("example=debug")
        .format(move |buffer, record| {
            let elapsed = (Instant::now() - start).as_secs_f64(); 
            writeln!(buffer, "[t={:>6.1}s] {}", elapsed, record.args())
        })
        .init();
}
