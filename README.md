## About

A few Rust tools for mirroring speedrun.com data.

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

### `verify`

Verifies that all of the scraped records can be deserialized into the
`api_types` we've defined. These types should preserve all of the details of
the original API data, even the buggy bits. This just confirms that we haven't
gotten any new data that has a structure we aren't prepared to handle.

### `normalize`

Converts the data into a normalized form you might put in a SQL database.

## Legal

### Disclosure

This project has no official relationship with speedrun.com.

### Code License

This code is released under the Blue Oak Model License 1.0.0, available at
<https://blueoakcouncil.org/license/1.0.0>.

### Content License

See <https://www.speedrun.com/legal> for details. All data is from speedrun.com
contributors, and is used and distributed under the Creative Commons
Attribution-NonCommercial 4.0 International license.
