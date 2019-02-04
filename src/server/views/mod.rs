#![warn(missing_debug_implementations, missing_docs)]
#![allow(unused_imports, missing_debug_implementations, missing_docs)]
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    error::Error,
    fs::File,
    io::{prelude::*, BufReader, BufWriter, Read},
    num::NonZeroU64 as Id64,
    ops::Deref,
    sync::Arc,
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

use crate::data::{database::Database, types::*};

pub trait View: Serialize + std::fmt::Debug {
    fn render(&self) -> Markup;

    fn html_to(&self, response: &mut hyper::Response<hyper::Body>) {
        response
            .headers_mut()
            .insert("Content-Type", HeaderValue::from_static("text/html"));

        let render = self.render().into_string();
        *response.body_mut() = Body::from(render);
    }
}

fn page(body: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        head {
            link charset="utf-8";
            link rel="stylesheet" href="/style.css";
            link rel="icon" href="/icon.gif";
            title { "SpeedRun.Com Archive" }
        }
        body {
            header {
                h1 {
                    a href="/" { "SpeedRun.Com Archive" }
                }
            }
            main {
                (body)
            }
            footer {
                p {
                    "This site is not affiliated with or endorsed by speedrun.com. "
                    "All data is from speedrun.com contributors, and is used and "
                    "distributed under the Creative Commons "
                    "Attribution-NonCommercial 4.0 International license."
                }
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Homepage;

impl<'db> View for Homepage {
    fn render(&self) -> Markup {
        page(html! {
            p {
                "Check out "
                a href="/celeste/anypercent" { "Celeste Any%" }
                "."
            }
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Debug<T: std::fmt::Debug + Serialize>(pub T);

impl<'db, T: std::fmt::Debug + Serialize> View for Debug<T> {
    fn render(&self) -> Markup {
        page(html! {
            pre {
                (format!("{:#?}", self.0))
            }
        })
    }
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
//         page(html!{
//             h1 {
//                 "unofficial speedrun.com data mirror"
//             }

//             @for run in &self.ranks {
//                 p {
//                     "#" (run.rank()) ". "
//                     (run.time_ms()) " ms"
//                 }
//             }
//         })
//     }
// }
