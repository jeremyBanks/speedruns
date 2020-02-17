#![allow(missing_docs, clippy::useless_attribute, clippy::useless_vec, unused)]
#![warn(missing_debug_implementations)]

use async_std::prelude::*;

use std::{collections::BTreeMap, rc::Rc, sync::Arc, time::Duration};

#[async_std::main]
async fn main() {
    #[allow(unused_mut)]
    let mut app = MyApp::new();

    loop {
        sleep(Duration::from_secs(5)).await;

        let request = "hello";

        async_std::task::spawn(app.handle_request(request));
    }
}

struct MyApp {
    database: Arc<Database>,
}

impl MyApp {
    fn new() -> Self {
        MyApp {
            database: Arc::new(Database::new()),
        }
    }

    // requirement: we need this to be able to keep running even if we update the
    // database, ideally in a consistent way.
    async fn handle_request(&self, request: &str) -> String {
        sleep(Duration::from_secs(5)).await;
        request.to_string()
    }
}

struct Database {
    table_user:         BTreeMap<u64, User>,
    index_user_by_name: BTreeMap<String, u64>,
}

impl Database {
    fn new() -> Self {
        Database {
            table_user:         Default::default(),
            index_user_by_name: Default::default(),
        }
    }
}

struct User {
    id:   u64,
    name: String,
}
