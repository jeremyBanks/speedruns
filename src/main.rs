#![feature(custom_attribute)]
#![feature(try_blocks)]

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_attributes)]

use reqwest;
use serde;
use serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate derive_more;

use std::{
    fs,
    convert::From,
};

pub fn main() -> Result<(), FatalError> {
    let json: Result<String, FatalError> = try {
        let wc2_runs_url = "https://www.speedrun.com/api/v1/runs?game=o1yry26q&orderby=date&direction=desc&status=verified&max=200";
        let mut response = reqwest::get(wc2_runs_url)?;
        if !response.status().is_success() {
            return Err(response.status().into());
        }
        let body = response.text()?;
        fs::write("../runs.json", &body)?;
        body
    };

    let json = match json {
        Ok(body) => body,
        Err(err) => {
            println!("Request failed, using cached JSON: {:?}", err);
            fs::read_to_string("../runs.json")?
        }
    };

    let parsed: speedruncom_api::runs::Response = serde_json::from_str(&json)?;
    let round = serde_json::to_string(&parsed)?;
    fs::write("runs-processed.json", &round)?;

    let mut runs: Vec<speedruncom_api::runs::Data> = parsed.data.into_iter().filter(|run| run.category == Some("wdmw5ee2".to_string())).collect();

    runs.sort_unstable_by_key(|run| run.times.primary_t);

    println!("{:#?}", runs.iter().next().unwrap());

    Ok(())
}

#[derive(Debug, From)]
pub enum FatalError {
    Http(reqwest::Error),
    HttpStatus(reqwest::StatusCode),
    Filesystem(std::io::Error),
    Json(serde_json::Error),
}

mod speedruncom_api {
    pub mod runs {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Response {
            pub data: Vec<Data>,
            pub pagination: super::Pagination,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Data {
            pub id: String,
            pub weblink: String,
            pub game: String,
            pub level: Option<String>,
            pub category: Option<String>,
            pub players: Vec<Player>,
            pub date: Option<String>,
            pub submitted: Option<String>,
            pub times: Times,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(tag = "rel")]
        #[serde(rename_all = "snake_case")]
        pub enum Player {
            User {
                id: String,
            },
            Guest {
                name: String,
            },
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Times {
            pub primary_t: u32,
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Pagination {
        pub offset: usize,
        pub max: usize,
        pub size: usize,
        pub links: Vec<Link>
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "rel")]
    #[serde(rename_all = "snake_case")]
    pub enum Link {
        Next {
            uri: String,
        }
    }
}
