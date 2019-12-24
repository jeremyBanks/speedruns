# speedruns

A few Rust tools for mirroring speedrun.com data.

[![on GitHub](https://img.shields.io/github/last-commit/m/jeremyBanks/speedruns.svg?label=github)](https://github.com/jeremyBanks/speedruns/)
[![on crates.io](https://img.shields.io/crates/v/speedruns.svg)](https://crates.io/crates/speedruns/)
[![on docs.rs](https://docs.rs/speedruns/badge.svg)](https://docs.rs/speedruns/)

## Usage

Optionally, download archived copies of `{runs,users,games}.jsonl.gz` from
https://archive.org/download/speedrun.com-2019-04-27 and put them in `data/api`.
If you skip this, you'll need to spend about a day download everything from the
API yourself.

Run `cargo run --release --bin scrape` to download any new data. This doesn't
include changes or deletions of existing items.

Run `cargo run --release --bin normalize` to normalize the data, including only
essential fields and records (no rejected or pending runs), and discarding
anything that fails validation or integrity checking. (Cargo might not detect
that the data has changed unless you run `cargo clean` after this step.)

Run `cargo build --release --run serve` to build and run the server binary,
which bundles in a compressed copy of the normalized data generated we generated
with `normalize`.

## Data

- `speedruns/data/`

  - `api/`  
    Records from speedrun.com API responses, as JSON, one per line, gzipped.

    - `games.jsonl.gz`  
      Embeds levels, categories, variables, gametypes, platforms, regions,
      genres, engines, developers, and publishers.
    - `users.jsonl.gz`
    - `runs.jsonl.gz`

  - `normalized/`  
    Our smaller normalized version of the speedrun.com records, one per line,
    uncompressed.

    - `games.jsonl`
    - `categories.jsonl`
    - `levels.jsonl`
    - `users.jsonl`
    - `runs.jsonl`

    We also save them in a LZMA-compressed unstable internal format only useful
    for bundling into the `serve` binary.

    - `games.bin.xz`
    - `categories.bin.xz`
    - `levels.bin.xz`
    - `users.bin.xz`
    - `runs.bin.xz`

## Legal

### Disclosure

This project is not associated with or endorsed by speedrun.com.

### Code License

Originally Copyright Jeremy Banks, released under the MIT License. Any
contributions from other authors must be released under both the MIT License and
the Blue Oak Model License 1.0.0.

### Content License

See <https://www.speedrun.com/legal> for details. All data is from speedrun.com
contributors, and is used and distributed under the Creative Commons
Attribution-NonCommercial 4.0 International license.
