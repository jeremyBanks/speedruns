#![feature(never_type, try_blocks)]
#![allow(unused_imports, dead_code)]
use flate2::{read::GzDecoder, write::GzEncoder};
use getset::{Getters, MutGetters, Setters};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::{map::Map as JsonMap, Value as JsonValue};
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
};
use tempfile::NamedTempFile;

pub type DynError = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Default)]
struct Spider {
    path: String,

    #[serde(default)]
    games_spidered_ranges: Vec<SpideredRange>,
    #[serde(default)]
    games_by_id: BTreeMap<String, JsonValue>,

    #[serde(default)]
    users_spidered_ranges: Vec<SpideredRange>,
    #[serde(default)]
    users_by_id: BTreeMap<String, JsonValue>,

    #[serde(default)]
    runs_spidered_ranges: Vec<SpideredRange>,
    #[serde(default)]
    runs_by_id: BTreeMap<String, JsonValue>,
}

#[derive(
    Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Serialize, Deserialize,
)]
struct SpideredRange {
    exclusive_start: chrono::DateTime<chrono::Utc>,
    exclusive_end: chrono::DateTime<chrono::Utc>,
}

trait SpideredModel {}

impl Spider {
    pub fn new(path: String) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub fn load_or_create(path: &str) -> Self {
        let spider: Result<Spider, DynError> = try {
            let file = File::open(&path)?;
            let buffer = BufReader::new(&file);
            let decompressor = GzDecoder::new(buffer);
            let spider: Spider = serde_json::from_reader(decompressor)
                .expect("if we can read the file it should be valid");
            spider
        };

        spider.unwrap_or(Spider::new(path.to_string()))
    }

    fn save(&mut self) -> Result<(), DynError> {
        // for each type,
        // get new records from the beginning until you start to see duplicates,
        // then start getting records from len until you reach the end
        trace!("Saving single json blob...");
        {
            let mut file = NamedTempFile::new_in("data")?;
            {
                let buffer = BufWriter::new(&mut file);
                let mut compressor = GzEncoder::new(buffer, Default::default());
                serde_json::to_writer(&mut compressor, &self)?;
                compressor.finish()?;
            }
            trace!("Compressed, flushing to disk...");
            file.persist(&self.path)?;
        }
        trace!("Saved.");

        trace!("Saving Games as Gzipped JSON Lines...");
        {
            let mut file = NamedTempFile::new_in("data")?;
            {
                let buffer = BufWriter::new(&mut file);
                let mut compressor = GzEncoder::new(buffer, Default::default());
                for game_data in self.games_by_id.values() {
                    serde_json::to_writer(&mut compressor, &game_data)?;
                    compressor.write(b"\n")?;
                }
                compressor.finish()?;
            }
            trace!("Compressed, flushing to disk...");
            file.persist("data/games.jsonl.gz")?;
        }
        trace!("Saved.");

        trace!("Saving Users as Gzipped JSON Lines...");
        {
            let mut file = NamedTempFile::new_in("data")?;
            {
                let buffer = BufWriter::new(&mut file);
                let mut compressor = GzEncoder::new(buffer, Default::default());
                for user_data in self.users_by_id.values() {
                    serde_json::to_writer(&mut compressor, &user_data)?;
                    compressor.write(b"\n")?;
                }
                compressor.finish()?;
            }
            trace!("Compressed, flushing to disk...");
            file.persist("data/users.jsonl.gz")?;
        }
        trace!("Saved.");

        trace!("Saving Runs as Gzipped JSON Lines...");
        {
            let mut file = NamedTempFile::new_in("data")?;
            {
                let buffer = BufWriter::new(&mut file);
                let mut compressor = GzEncoder::new(buffer, Default::default());
                for run_data in self.runs_by_id.values() {
                    serde_json::to_writer(&mut compressor, &run_data)?;
                    compressor.write(b"\n")?;
                }
            }
            trace!("Compressed, flushing to disk...");
            file.persist("data/run.jsonl.gz")?;
        }
        trace!("Saved.");

        Ok(())
    }

    pub fn run(&mut self) -> Result<!, DynError> {
        let mut previous = self.games_by_id.len();
        for i in 0..=std::usize::MAX {
            let len = self.games_by_id.len();

            info!("Got {} games, fetching more...", len);

            let url = format!("https://www.speedrun.com/api/v1/games?direction=desc&embed=levels,categories,variables,gametypes,platforms,regions,genres,engines,developers,publishers&max=200&orderby=created&offset={}", len);
            let response: JsonValue = reqwest::get(&url)
                .expect("the request to succeed")
                .json()
                .unwrap();
            let response = response.as_object().unwrap();
            let games = response["data"].as_array().unwrap();

            for game in games.iter().cloned() {
                let id = game.get("id").unwrap().as_str().unwrap().to_string();
                self.games_by_id.insert(id, game);
            }

            if self.games_by_id.len() == previous {
                info!("got all of the games!");
                break;
            } else {
                previous = self.games_by_id.len();
            }

            if i % 64 == 0 {
                self.save()?;
            }

            std::thread::sleep(std::time::Duration::from_millis(600));
        }

        let mut previous = self.users_by_id.len();

        for i in 0..=std::usize::MAX {
            let len = self.users_by_id.len();

            info!("Got {} users, fetching more...", len);

            let url = format!("https://www.speedrun.com/api/v1/users?direction=desc&max=200&orderby=signup&offset={}", len);
            let response: JsonValue = reqwest::get(&url)
                .expect("the request to succeed")
                .json()
                .unwrap();
            let response = response.as_object().unwrap();
            let users = response["data"].as_array().unwrap();

            for user in users.iter().cloned() {
                let id = user.get("id").unwrap().as_str().unwrap().to_string();
                self.users_by_id.insert(id, user);
            }

            // TODO: Check for pagination.size being less than the max amount of
            // elements you want to detect the last page.

            if self.users_by_id.len() == previous {
                info!("got all of the users!");
                break;
            } else {
                previous = self.users_by_id.len();
            }

            if i % 64 == 0 {
                self.save()?;
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        let mut previous = self.runs_by_id.len();

        for outer in vec![0usize, 1] {
            for i in 0..=std::usize::MAX {
                let len = self.runs_by_id.len();

                let offset = if outer == 0 {
                    // first check for new runs
                    i * 200
                } else {
                    // then check for missed trailing runs
                    len
                };

                info!("Got {} runs, fetching more...", len);

                let url = format!("https://www.speedrun.com/api/v1/runs?direction=desc&max=200&orderby=submitted&offset={}", offset);
                let response: JsonValue = reqwest::get(&url)
                    .expect("the request to succeed")
                    .json()
                    .unwrap();
                let response = response.as_object().unwrap();
                let runs = response["data"].as_array().unwrap();

                for run in runs.iter().cloned() {
                    let id =
                        run.get("id").unwrap().as_str().unwrap().to_string();
                    self.runs_by_id.insert(id, run);
                }

                let more = self.runs_by_id.len() - previous;
                trace!("Got {} more.", more);

                if outer == 0 {
                    if self.runs_by_id.len() == previous {
                        // no new runs at beginning of list
                        break;
                    }
                } else {
                    if runs.len() < 200 {
                        // end of entire run list
                        break;
                    }
                };

                previous = self.runs_by_id.len();

                if i % 64 == 0 {
                    self.save()?;
                }

                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }

        self.save()?;

        std::process::exit(0)
    }
}

fn main() -> Result<!, DynError> {
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("RUST_LOG", "reqwest=debug,scrape=trace"),
    );

    let mut spider = Spider::load_or_create("data/state.json.gz");
    spider.run()
}
