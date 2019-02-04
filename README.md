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

- `data/users.jsonl.gz`
- `data/games.jsonl.gz`
- `data/runs.jsonl.gz`
- `data/scrape.json`

### `structure`

Creates a SQLite database from mirrored JSON data. JSON fields that we don't
convert to SQL columns or tables will be preserved together in an `extra`
JSON string column on the `User`, `Game`, and `Run` tables.

- `data/speedrun.sqlite3`

### `serve`

Provides a simple web interface for viewing the SQLite database.

## Legal

### Disclosure

This project has no official relationship with speedrun.com.

### Code License

This code is released into the public domain to the extent permissible by law,
under the terms of the Creative Commons CC0 declaration.

### Content License

See https://www.speedrun.com/legal for details. All data is from speedrun.com
contributors, and is used and distributed under the Creative Commons
Attribution-NonCommercial 4.0 International license.
