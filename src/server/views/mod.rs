use std::io::Write;

use flate2::write::GzEncoder;
use hyper::{header::HeaderValue, Body};
#[allow(unused)] use log::{debug, error, info, trace, warn};
use maud::{html, Markup};
use serde::Serialize;

use crate::data::{database::Linked, leaderboard::RankedRun, types::*};

pub trait View: Serialize + std::fmt::Debug {
    fn render(&self) -> Markup;

    fn html_to(&self, response: &mut hyper::Response<hyper::Body>) {
        let headers = response.headers_mut();
        headers.insert("Content-Type", HeaderValue::from_static("text/html"));
        headers.insert("Content-Encoding", HeaderValue::from_static("gzip"));

        let render = self.render().into_string();

        let mut buffer = Vec::<u8>::new();
        let mut compressor = GzEncoder::new(&mut buffer, flate2::Compression::best());
        compressor.write_all(render.as_bytes()).unwrap();
        compressor.finish().unwrap();

        *response.body_mut() = Body::from(buffer);
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
    pub game:     Linked<Game>,
    pub category: Linked<Category>,
    pub level:    Option<Linked<Level>>,
    pub ranks:    Vec<RankedRun>,
}

impl<'db> View for LeaderboardPage {
    fn render(&self) -> Markup {
        page(html! {
            h2 {
                (self.game.name())
            }
            h3 {
                (self.category.name())
            }
            @if let Some(level) = &self.level {
                h4 {
                    (level.name())
                }
            }
            @for rank in &self.ranks {
                p {
                    "#" (rank.rank()) ". "
                    (rank.time_ms()) " ms"
                    " by " (format!("{:#?}", rank.run().users().to_vec().iter().map(|u| u.name()).collect::<Vec<_>>()))
                }
            }
        })
    }
}
