#![feature(arbitrary_self_types)]
#![allow(unused)]
use async_std::prelude::*;
use rand::prelude::*;
use std::{collections::BTreeMap, rc::Rc, sync::Arc, time::Duration};

#[derive(Debug)]
struct Request {}

#[derive(Debug)]
struct Response {}

#[derive(Debug)]
struct WebApp {}

impl WebApp {
    fn new() -> WebApp {
        WebApp {}
    }

    async fn handle(self: Arc<Self>, request: Request) -> Response {
        // Pretend we're waiting for a slow backend.
        async_std::task::sleep(Duration::from_secs(5)).await;
        Response {}
    }
}

#[async_std::main]
async fn main() {
    let app = Arc::new(WebApp::new());

    loop {
        // For some variety, let's say that every one second...
        async_std::task::sleep(Duration::from_secs(1)).await;
        // ...we have a 20% probability...
        if 0.20 > rand::thread_rng().gen_range(0.0, 1.0) {
            // ...of simulating a request.
            let request = Request {};
            println!(" request: {:?}", request);

            let app_for_task = app.clone();
            async_std::task::spawn(async {
                let response = app_for_task.handle(request).await;
                println!("response: {:?}", response);
            });
        }
    }
}
