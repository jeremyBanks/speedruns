#![warn(missing_debug_implementations, missing_docs)]
#![allow(unused_imports, missing_debug_implementations, missing_docs)]
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    error::Error,
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader, BufWriter, Read},
    num::NonZeroU64 as Id64,
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
use maud::{html, Markup, PreEscaped, Render};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::Validate;
use xz2::read::XzDecoder;

use crate::data::{base::Database, types::*};

pub trait View: Serialize + Debug {
    fn render(&self) -> Markup;
}

// #[derive(Debug, Serialize)]
// pub struct LeaderboardPage {
//     pub game:     &'static Game,
//     pub category: &'static Category,
//     pub level:    Option<&'static Level>,
//     pub ranks:    Vec<RankedRun>,
// }

// impl<'db> View for LeaderboardPage {
//     fn render(&self) -> Markup {
//         html! {
//             (maud::DOCTYPE)
//             head {
//                 title {
//                     "speedruns"
//                 }

//                 style { r"
//                     body { font-family: sans-serif; }
//                     pre { white-space: pre-wrap; }
//                 " }
//             }

//             body {
//                 h1 {
//                     "unofficial speedrun.com data mirror"
//                 }

//                 @for run in &self.ranks {
//                     p {
//                         "#" (run.rank()) ". "
//                         (run.time_ms()) " ms"
//                     }
//                 }
//             }
//         }
//     }
// }
