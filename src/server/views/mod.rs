use std::io::Write;

use chrono::NaiveDate;
use flate2::write::GzEncoder;
use hyper::{header::HeaderValue, Body};
#[allow(unused)] use log::{debug, error, info, trace, warn};
use maud::{html, Markup};
use serde::Serialize;
use serde_json;

use crate::{
    data::{database::Linked, leaderboard::RankedRun, types::*},
    server::path::Path,
};

pub trait View: Serialize + std::fmt::Debug {
    fn render(&self) -> Markup;

    fn write_response(&self, response: &mut hyper::Response<hyper::Body>, as_json: bool) {
        let string: String;
        if !as_json {
            response
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("text/html"));

            string = self.render().into_string();
        } else {
            string = serde_json::to_string_pretty(&self).unwrap();
        }

        response
            .headers_mut()
            .insert("Content-Encoding", HeaderValue::from_static("gzip"));
        let mut buffer = Vec::<u8>::new();
        let mut compressor = GzEncoder::new(&mut buffer, flate2::Compression::best());
        compressor.write_all(string.as_bytes()).unwrap();
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
            link rel="icon" href="/srca.gif";
            title { "SpeedRun.Com Archive" }
        }
        body {
            header {
                h1 {
                    a href="/" {
                        img src="/srca.gif" alt="" style="height: 1em";
                        "SpeedRun.Com Archive"
                    }
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
                a href="celeste/clear/forsaken-city" { "celeste/clear/forsaken-city" }
                "."
            }
        })
    }
}

#[allow(unused)]
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
                ": "
                (self.category.name())
                @if let Some(level) = &self.level {
                    ": "
                    (level.name())
                }
                " "
                a href="https://www.speedrun.com/Celeste/Forsaken_City#Clear" {
                    img src="/src.png" alt="on speedrun.com" style="height: 1em";
                }
                " "
                a href="/celeste/clear/forsaken-city.json" {
                    code {
                        "{…}"
                    }
                }
            }
            table {
                thead {
                    tr {
                        th class="rank" { "rank" }
                        th class="time" { "time" }
                        th class="date" { "date" }
                        th class="runner" { "runner" }
                    }
                }
                tbody {
                    @for rank in &self.ranks {
                        tr data-rank=(rank.tied_rank()) {
                            th class="rank" {
                                (rank.tied_rank())
                            }
                            td class="time" {
                                (render_time(*rank.time_ms()))
                            }
                            td class="date" {
                                a href=(Path::Run(rank.run().clone()).to_string()) {
                                    (rank.run().date().render())
                                }
                            }
                            td class="runner" {
                                @for user in rank.run().users() {
                                    (user.render())
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

fn render_time(ms_total: u64) -> String {
    let mut pieces = Vec::<String>::new();
    let ms = ms_total % 1000;
    let s_total = ms_total / 1000;
    let s = s_total % 60;
    let m_total = s_total / 60;
    let m = m_total % 60;
    let h_total = m_total / 60;
    let h = h_total;

    if h_total > 0 {
        pieces.push(format!("{}:", h));
    }

    if m_total > 0 {
        if h_total > 0 {
            pieces.push(format!("{:02}:", m));
        } else {
            pieces.push(format!("{}:", m));
        }
    }

    if m_total > 0 {
        pieces.push(format!("{:02}", s));
    } else {
        pieces.push(format!("{}", s));
    }

    if ms > 0 {
        pieces.push(format!(".{:03}", ms));
    } else {
        pieces.push("    ".to_string());
    }

    pieces.join("")
}

impl View for Linked<User> {
    fn render(&self) -> Markup {
        html! {
            a href=(Path::User(self.clone()).to_string()) {
                (self.name())
            }
        }
    }
}

impl View for Option<NaiveDate> {
    fn render(&self) -> Markup {
        html! {
            @if let Some(date) = self {
                (date.format("%Y-%m-%d").to_string())
            }
        }
    }
}
