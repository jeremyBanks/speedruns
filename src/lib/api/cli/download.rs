#![allow(clippy::useless_attribute, clippy::useless_vec)]

use chrono::Local;
use flate2::{read::GzDecoder, write::GzEncoder};

use log::{debug, error, info, warn};
use rand::seq::SliceRandom;
use rand::prelude::*;
use serde_json::{Deserializer as JsonDeserializer, Value as JsonValue};
use std::{
    collections::{BTreeMap, HashSet},
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

// Only games and runs support bulk listing; users must be fetched individually
const BULK_RESOURCES: [Resource; 2] = [
    Resource {
        id: "games",
        order: "created",
        embed: "levels,categories,variables,gametypes,platforms,regions,genres,engines,developers,publishers"
    },
    Resource {
        id: "runs",
        order: "submitted",
        embed: ""
    },
];

// Dummy resource for users (used for load/save only, not bulk download)
const USERS_RESOURCE: Resource = Resource {
    id: "users",
    order: "",
    embed: ""
};

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
        let all_resources = [&BULK_RESOURCES[0], &BULK_RESOURCES[1], &USERS_RESOURCE];
        for resource in all_resources.iter() {
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
                let mut count = 0;
                for item in iterator {
                    let item = item?;
                    let id = item
                        .get("id")
                        .unwrap()
                        .as_str()
                        .expect("record should have id field")
                        .to_string();
                    spider.resource_by_id(resource).insert(id, item);
                    count += 1;
                    if count % 50000 == 0 {
                        debug!("  ...loaded {} {} so far", count, resource.id);
                    }
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
                let total = self.resource_by_id(resource).len();
                let mut count = 0;
                for data in self.resource_by_id(resource).values() {
                    serde_json::to_writer(&mut compressor, &data)?;
                    compressor.write_all(b"\n")?;
                    count += 1;
                    if count % 100000 == 0 {
                        debug!("  ...saved {}/{} {}", count, total, resource.id);
                    }
                }
                compressor.finish()?;
            }
            debug!("Persisting temp file...");
            file.persist(format!("{}/{}.jsonl.gz", DATA_API_DIR, resource.id))?;
        }
        info!("Saved.");

        Ok(())
    }

    /// Backup existing data files with YYYYMMDDHHMM- prefix
    fn backup_data_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = Local::now().format("%Y%m%d%H%M").to_string();

        for name in &["games", "runs", "users"] {
            let src = format!("{}/{}.jsonl.gz", DATA_API_DIR, name);
            if Path::new(&src).exists() {
                let dst = format!("{}/{}-{}.jsonl.gz", DATA_API_DIR, timestamp, name);
                info!("Backing up {} -> {}", src, dst);
                fs::copy(&src, &dst)?;
            }
        }

        Ok(())
    }

    pub async fn run(&mut self, limit: i32, backup: bool) -> Result<(), Box<dyn std::error::Error>> {
        if backup {
            self.backup_data_files()?;
        }
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

        // --limit mode: fetch runs for N random games instead of bulk download
        if limit >= 0 {
            return self.run_limited(&client, limit as usize).await;
        }

        for resource in BULK_RESOURCES.iter() {
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

                    // save progress periodically (every 256 pages = ~51K items)
                    if i > 0 && i % 256 == 0 {
                        self.save(resource)?;
                    }

                    previous = self.resource_by_id(resource).len();

                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }

            self.save(resource)?;
        }

        // After downloading runs, fetch any missing users referenced in runs
        self.fetch_missing_users(&client).await?;

        Ok(())
    }

    /// --limit mode: fetch runs for N random games
    async fn run_limited(&mut self, client: &reqwest::Client, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
        if limit == 0 {
            info!("Limit is 0, nothing to do.");
            return Ok(());
        }

        if self.games_by_id.is_empty() {
            error!("No games loaded. Run a full download first to get the games list.");
            return Ok(());
        }

        // Pick N random games
        let mut game_ids: Vec<String> = self.games_by_id.keys().cloned().collect();
        game_ids.shuffle(&mut rand::thread_rng());
        let selected_games: Vec<String> = game_ids.into_iter().take(limit).collect();

        info!("Selected {} game(s) to fetch runs for.", selected_games.len());

        let mut all_fetched_game_ids: Vec<String> = Vec::new();

        for (i, game_id) in selected_games.iter().enumerate() {
            let game_name = self.games_by_id.get(game_id)
                .and_then(|g| g.get("names"))
                .and_then(|n| n.get("international"))
                .and_then(|n| n.as_str())
                .unwrap_or("unknown");

            info!("[{}/{}] Selected game: {} ({})", i + 1, limit, game_name, game_id);

            // Count runs we have for this game before fetching
            let runs_before: usize = self.runs_by_id.values()
                .filter(|r| r.get("game").and_then(|g| g.as_str()) == Some(game_id.as_str()))
                .count();
            info!("We have {} runs for this game.", runs_before);

            // Fetch runs for this game
            let mut offset = 0;
            loop {
                let url = format!(
                    "https://www.speedrun.com/api/v1/runs?game={}&max=200&offset={}",
                    game_id, offset
                );

                info!("Fetching runs for {} (offset {})...", game_name, offset);

                let response_data: JsonValue = match client.get(&url).send().await {
                    Ok(response) => {
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
                        match response.json().await {
                            Ok(j) => j,
                            Err(e) => {
                                error!("Failed to parse response: {:?}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Request failed: {:?}", e);
                        break;
                    }
                };

                let items = response_data
                    .get("data")
                    .and_then(|d| d.as_array())
                    .map(|a| a.to_vec())
                    .unwrap_or_default();

                let count = items.len();
                for item in items {
                    if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
                        self.runs_by_id.insert(id.to_string(), item);
                    }
                }

                info!("Got {} runs.", count);

                if count < 200 {
                    // End of runs for this game
                    break;
                }

                offset += 200;
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }

            let runs_after: usize = self.runs_by_id.values()
                .filter(|r| r.get("game").and_then(|g| g.as_str()) == Some(game_id.as_str()))
                .count();

            info!("Finished fetching runs for {}. Had {}, now have {} ({} new).",
                  game_name, runs_before, runs_after, runs_after - runs_before);

            all_fetched_game_ids.push(game_id.clone());
        }

        // Save runs
        self.save(&BULK_RESOURCES[1])?; // runs

        // Fetch missing users from all the games we just fetched
        for game_id in &all_fetched_game_ids {
            self.fetch_missing_users_for_game(client, game_id).await?;
        }

        Ok(())
    }

    /// Fetch missing users referenced in runs for a specific game
    async fn fetch_missing_users_for_game(&mut self, client: &reqwest::Client, game_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Collect user IDs from runs for this game
        let mut user_ids: HashSet<String> = HashSet::new();
        for run in self.runs_by_id.values() {
            if run.get("game").and_then(|g| g.as_str()) != Some(game_id) {
                continue;
            }
            if let Some(players) = run.get("players").and_then(|p| p.as_array()) {
                for player in players {
                    if let Some(id) = player.get("id").and_then(|v| v.as_str()) {
                        user_ids.insert(id.to_string());
                    }
                }
            }
        }

        // Find missing users
        let missing: Vec<String> = user_ids
            .iter()
            .filter(|id| !self.users_by_id.contains_key(*id))
            .cloned()
            .collect();

        if missing.is_empty() {
            info!("All users for this game already present.");
            return Ok(());
        }

        info!("Fetching {} missing users for this game...", missing.len());

        let mut fetched = 0;
        for user_id in &missing {
            let url = format!("https://www.speedrun.com/api/v1/users/{}", user_id);

            match client.get(&url).send().await {
                Ok(response) => {
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

                    if !response.status().is_success() {
                        warn!("Failed to fetch user {}: {}", user_id, response.status());
                        continue;
                    }

                    if let Ok(json) = response.json::<JsonValue>().await {
                        if let Some(user_data) = json.get("data") {
                            self.users_by_id.insert(user_id.clone(), user_data.clone());
                            fetched += 1;
                        }
                    }
                }
                Err(e) => {
                    warn!("Request failed for user {}: {:?}", user_id, e);
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }

        info!("Fetched {} new users.", fetched);
        if fetched > 0 {
            self.save(&USERS_RESOURCE)?;
        }

        Ok(())
    }

    /// Extract user IDs from all runs and fetch any we don't already have
    async fn fetch_missing_users(&mut self, client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
        info!("Checking for missing users referenced in runs...");

        // Collect all user IDs referenced in runs
        let mut referenced_user_ids: HashSet<String> = HashSet::new();
        for run in self.runs_by_id.values() {
            if let Some(players) = run.get("players").and_then(|p| p.as_array()) {
                for player in players {
                    // Players can be either user references or guests
                    if let Some(id) = player.get("id").and_then(|v| v.as_str()) {
                        referenced_user_ids.insert(id.to_string());
                    }
                }
            }
        }

        // Find users we don't have yet
        let existing_user_ids: HashSet<&String> = self.users_by_id.keys().collect();
        let missing_user_ids: Vec<String> = referenced_user_ids
            .iter()
            .filter(|id| !existing_user_ids.contains(id))
            .cloned()
            .collect();

        if missing_user_ids.is_empty() {
            info!("All referenced users already present.");
            return Ok(());
        }

        info!("Need to fetch {} missing users.", missing_user_ids.len());

        let mut fetched = 0;
        let mut failed = 0;
        for user_id in &missing_user_ids {
            let url = format!("https://www.speedrun.com/api/v1/users/{}", user_id);

            match client.get(&url).send().await {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        let retry_after = response
                            .headers()
                            .get(reqwest::header::RETRY_AFTER)
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.parse::<u64>().ok())
                            .unwrap_or(60);
                        error!("Rate limited, sleeping for {} seconds", retry_after);
                        tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                        // Don't increment failed, will retry on next run
                        continue;
                    }

                    if response.status() == reqwest::StatusCode::NOT_FOUND {
                        warn!("User {} not found (may have been deleted)", user_id);
                        failed += 1;
                        continue;
                    }

                    if !response.status().is_success() {
                        warn!("Failed to fetch user {}: {}", user_id, response.status());
                        failed += 1;
                        continue;
                    }

                    match response.json::<JsonValue>().await {
                        Ok(json_response) => {
                            if let Some(user_data) = json_response.get("data") {
                                self.users_by_id.insert(user_id.clone(), user_data.clone());
                                fetched += 1;
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse user {}: {:?}", user_id, e);
                            failed += 1;
                        }
                    }
                }
                Err(e) => {
                    warn!("Request failed for user {}: {:?}", user_id, e);
                    failed += 1;
                }
            }

            // Save progress periodically
            if fetched % 100 == 0 && fetched > 0 {
                info!("Fetched {} users so far...", fetched);
                self.save(&USERS_RESOURCE)?;
            }

            // Rate limit ourselves
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }

        info!("Fetched {} new users, {} failed.", fetched, failed);
        self.save(&USERS_RESOURCE)?;

        Ok(())
    }
}

pub async fn main(limit: i32, backup: bool) -> Result<(), Box<dyn std::error::Error>> {
    Spider::load_or_create().run(limit, backup).await
}
