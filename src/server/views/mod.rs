use hyper::{header::HeaderValue, Body};
#[allow(unused)] use log::{debug, error, info, trace, warn};
use maud::{html, Markup};
use serde::Serialize;

use crate::data::{leaderboard::RankedRun, types::*};

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
            meta charset="utf-8";
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

#[derive(Debug, Serialize)]
pub struct LeaderboardPage {
    pub game:     &'static Game,
    pub category: &'static Category,
    pub level:    Option<&'static Level>,
    pub ranks:    Vec<RankedRun>,
}

impl<'db> View for LeaderboardPage {
    fn render(&self) -> Markup {
        page(html! {
            @for run in &self.ranks {
                p {
                    "#" (run.rank()) ". "
                    (run.time_ms()) " ms"
                }
            }
        })
    }
}
