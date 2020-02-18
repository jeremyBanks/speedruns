#![feature(arbitrary_self_types, assoc_int_consts)]
#![allow(unused_imports)]
use async_std::{
    prelude::*,
    sync::{channel, Arc, RwLock},
    task::{sleep, spawn},
};
use log::{debug, error, info, trace, warn};
use rand::prelude::*;
use std::{
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
    names: RwLock<Arc<HashSet<String>>>,
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
        let mut new_names = (**names).clone();
        new_names.insert(name);
        *names = Arc::new(new_names);
        sleep(Duration::from_secs_f64(2.25)).await;
    }

    pub async fn remove_name(&self, name: String) {
        debug!("database: remove_name({:?}) requesting write lock", name);
        let mut names = self.names.write().await;
        debug!("database: remove_name({:?}) got write lock", name);
        let mut new_names = (**names).clone();
        new_names.remove(&name);
        *names = Arc::new(new_names);
        sleep(Duration::from_secs_f64(4.25)).await;
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

        for _ in 0..2 {
            sleep(Duration::from_secs_f64(1.0)).await;

            let request = Request::GetNames;
            sender.send(request).await;
        }
    });

    let server = spawn(async move {
        let mut tasks: Vec<Box<dyn Future<Output = ()>>> = Vec::new();

        loop {
            let request = match receiver.recv().await {
                Some(request) => request,
                None => {
                    debug!("server: connection channel closed");
                    break
                }
            };
            debug!("server: got request {:?}", request);

            let app_for_task = app.clone();
            tasks.push(Box::new(spawn(async {
                let request_s = format!("{:?}", request);

                let response = app_for_task.handle(request).await;
                debug!(
                    "server: sent response {:?} for request {}",
                    response, request_s
                );
            })));
        }

        debug!("server shutting down, now blocking on remaining tasks");
        let mut all_tasks: Option<Box<dyn Future<Output = ()>>> = None;
        for task in tasks {
            let new_tasks = if let Some(prev_tasks) = all_tasks.take() {
                prev_tasks.join(task)
            } else {
                task
            };
            all_tasks = Some(new_tasks);
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
