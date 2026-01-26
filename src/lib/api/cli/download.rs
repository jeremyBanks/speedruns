#![allow(clippy::useless_attribute, clippy::useless_vec)]

use flate2::{read::GzDecoder, write::GzEncoder};

use log::{debug, error, info};
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{prelude::*, BufReader, BufWriter},
    path::Path,
};
use tempfile::NamedTempFile;

const DATA_API_DIR: &str = "data/api";

#[derive(PartialEq, Eq, Hash)]
struct Resource {
    id: &'static str,
    order: &'static str,
    embed: &'static str,
}

const RESOURCES: [Resource; 3] = [
    Resource {
        id: "games",
        order: "created",
        embed: "levels,categories,variables,gametypes,platforms,regions,genres,engines,developers,publishers"
    },
    Resource {
        id: "users",
        order: "signup",
        embed: ""
    },
    Resource {
        id: "runs",
        order: "submitted",
        embed: ""
    },
];

#[derive(Default)]
struct Spider {
    games_by_id: BTreeMap<String, JsonValue>,
    users_by_id: BTreeMap<String, JsonValue>,
    runs_by_id: BTreeMap<String, JsonValue>,
}

impl Spider {
    fn resource_by_id(&mut self, resource: &Resource) -> &mut BTreeMap<String, JsonValue> {
        match resource.id {
            "runs" => &mut self.runs_by_id,
            "games" => &mut self.games_by_id,
            "users" => &mut self.users_by_id,
            _ => unreachable!(),
        }
    }

    pub fn load_or_create() -> Self {
        let mut spider = Spider::default();

        // Ensure data directory exists
        if let Err(e) = fs::create_dir_all(DATA_API_DIR) {
            error!("Failed to create data directory {}: {:?}", DATA_API_DIR, e);
        }

        // Load each resource independently - missing files are OK for fresh start
        for resource in RESOURCES.iter() {
            let path = format!("{}/{}.jsonl.gz", DATA_API_DIR, resource.id);
            if !Path::new(&path).exists() {
                info!("No existing {} data at {}, starting fresh.", resource.id, path);
                continue;
            }

            info!("Loading {}...", resource.id);
            let mut load_resource = || -> Result<usize, Box<dyn std::error::Error>> {
                let file = File::open(&path)?;
                let buffer = BufReader::new(&file);
                let decompressor = GzDecoder::new(buffer);
                let deserializer = JsonDeserializer::from_reader(decompressor);
                let iterator = deserializer.into_iter::<JsonValue>();
                for item in iterator {
                    let item = item?;
                    let id = item
                        .get("id")
                        .unwrap()
                        .as_str()
                        .expect("record should have id field")
                        .to_string();
                    spider.resource_by_id(resource).insert(id, item);
                }
                Ok(spider.resource_by_id(resource).len())
            };

            match load_resource() {
                Ok(count) => info!("Loaded {} {}.", count, resource.id),
                Err(e) => error!("Failed to load {}: {:?}", resource.id, e),
            }
        }

        spider
    }

    fn save(&mut self, resource: &Resource) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure directory exists before saving
        fs::create_dir_all(DATA_API_DIR)?;

        info!(
            "Saving {} {}...",
            self.resource_by_id(resource).len(),
            resource.id
        );
        {
            let mut file = NamedTempFile::new_in(DATA_API_DIR)?;
            {
                let buffer = BufWriter::new(&mut file);
                let mut compressor = GzEncoder::new(buffer, flate2::Compression::best());
                for data in self.resource_by_id(resource).values() {
                    serde_json::to_writer(&mut compressor, &data)?;
                    compressor.write_all(b"\n")?;
                }
                compressor.finish()?;
            }
            file.persist(format!("{}/{}.jsonl.gz", DATA_API_DIR, resource.id))?;
        }
        info!("Saved.");

        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();

        let user_agent = format!(
            "{}/{}",
            option_env!("CARGO_PKG_NAME").unwrap_or("unknown"),
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
        );

        debug!("user agent: {}", user_agent);

        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_str(&user_agent)?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        for resource in RESOURCES.iter() {
            // the logic:
            // try to grab from offset of len
            // if you see any duplicates, that means you're missing some at
            // the beginning, so you need to switch back into that mode.
            // if you don't see any duplicates, keep going forward until
            // you get a non-full page, indicating that you're at the end.
            // you can't save while you're filling from the beginning,
            // because if that's interrupted you could create gaps.
            //
            // Hmm, actually, I guess you can find gaps, eh?
            // If your end count is wrong but there are no new items at the
            // beginning, you can do a binary search to find the
            // place that missing records throw off your indices.
            //
            // deletions still mess this up, though. you'd need to be able
            // to identify them to have a fullly robust solution.

            for from_start in vec![true, false] {
                let mut previous = self.resource_by_id(resource).len();
                for i in 0..=std::usize::MAX {
                    let resource_by_id = self.resource_by_id(resource);
                    let len = resource_by_id.len();

                    let offset = if from_start { i * 200 } else { len };

                    let what = if from_start { "new" } else { "old" };
                    info!(
                        "We have {} {}, looking for more {} {}...",
                        len, resource.id, what, resource.id
                    );

                    let url = format!("https://www.speedrun.com/api/v1/{}?direction=desc&max=200&orderby={}&embed={}&offset={}", resource.id, resource. order, resource.embed, offset);

                    let response_data: JsonValue;
                    loop {
                        match client.get(&url).send().await {
                            Ok(response) => {
                                // Check for rate limiting (429 Too Many Requests)
                                if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                                    let retry_after = response
                                        .headers()
                                        .get(reqwest::header::RETRY_AFTER)
                                        .and_then(|v| v.to_str().ok())
                                        .and_then(|v| v.parse::<u64>().ok())
                                        .unwrap_or(60);
                                    error!("Rate limited, sleeping for {} seconds", retry_after);
                                    tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                                    continue;
                                }

                                match response.json::<JsonValue>().await {
                                    Ok(json_response) => {
                                        response_data = json_response;
                                        break;
                                    }
                                    Err(error) => {
                                        error!("response error: {:?}", error);
                                        tokio::time::sleep(std::time::Duration::from_secs(32)).await;
                                        continue;
                                    }
                                }
                            },
                            Err(error) => {
                                error!("request error: {:?}", error);
                                tokio::time::sleep(std::time::Duration::from_secs(32)).await;
                                continue;
                            }
                        }
                    }

                    let response = response_data
                        .as_object()
                        .expect("json response to have expected structure");
                    let items = response["data"]
                        .as_array()
                        .expect("json response to have expected structure");

                    for item in items.iter().cloned() {
                        let id = item
                            .get("id")
                            .expect("json response to have expected structure")
                            .as_str()
                            .expect("json response to have expected structure")
                            .to_string();
                        self.resource_by_id(resource).insert(id, item);
                    }

                    let more = self.resource_by_id(resource).len() - previous;
                    info!("Got {} more {}.", more, resource.id);

                    if from_start {
                        if self.resource_by_id(resource).len() == previous {
                            // no new items at beginning of list
                            break;
                        }
                    } else if items.len() < 200 {
                        // end of entire run list
                        break;
                    };

                    // save progress
                    if i % 32 == 255 {
                        self.save(resource)?;
                    }

                    previous = self.resource_by_id(resource).len();

                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }

            self.save(resource)?;
        }

        Ok(())
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Spider::load_or_create().run().await
}
