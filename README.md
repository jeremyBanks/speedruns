## About

A few Rust tools for mirroring speedrun.com data.

## Data

You should start by download an existing data archive so you only need to pull
the newest records from speedrun.com. You can find one at
<https://archive.org/details/speedrun.com-2019-04-27>.

## Tools

You can run these through Cargo with `cargo run --release --bin NAME`.

### `scrape`

Create or update a local mirror of the public data from speedrun.com's API.
This will produce three files in the gzipped JSON Lines format: each line
corresponds to an top-level value in a speedrun.com API response, with the
neccessary embeddings enabled to ensure that other secondary data is also
captured.

- `data/api/users.jsonl.gz`
- `data/api/games.jsonl.gz`
- `data/api/runs.jsonl.gz`

This only looks for new records, it doesn't try to update existing records,s
so we currently won't know if a pending run becomes verified, or deleted.

### `normalize`

Converts the data into a normalized form you might put in a SQL database.
This excludes some of the metadata and rejected runs in the original data.

- `data/normalized/users.jsonl`
- `data/normalized/games.jsonl`
- `data/normalized/runs.jsonl`
- `data/normalized/levels.jsonl`
- `data/normalized/categories.jsonl`

### `leaderboards`

Serve a web view of the leaderboards.

## Legal

### Disclosure

This project has no official relationship with speedrun.com.

### Content License

See <https://www.speedrun.com/legal> for details. All data is from speedrun.com
contributors, and is used and distributed under the Creative Commons
Attribution-NonCommercial 4.0 International license.
