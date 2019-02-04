# speedruns

A few Rust tools for mirroring speedrun.com data.

[![on CircleCI](https://circleci.com/gh/jeremyBanks/speedruns/tree/master.svg?style=svg&circle-token=bc9b0cb90d9bab53eaebdf02a1afa7e4dc1b57ad)](https://circleci.com/gh/jeremyBanks/speedruns/tree/master)
[![on crates.io](https://img.shields.io/crates/v/speedruns.svg)](https://crates.io/crates/speedruns/)
[![on docs.rs](https://docs.rs/speedruns/badge.svg)](https://docs.rs/speedruns/)

## Usage

Optionally, download archived copies of `{runs,users,games}.jsonl.gz` from https://archive.org/download/speedrun.com-2019-04-27 and put them in `data/api`. If you skip this, you'll need to spend about a day download everything from the API yourself. (Note that if you do use archived data, you won't see any deletions or changes from after that archive was published.)

Run `cargo run --release --bin --scrape` to download any new data.

Run `cargo run --release --bin normalize` to normalize the data, including only essential fields and records, and discarding anything that fails validation or integrity checking.

Run `cargo build --release --run serve` to build and run the server binary, which bundling in a compressed copy of the normalized data generated we generated with `normalize`.

## Current Structure

The `speedrun` package interface is not stable and may never be, but the tools should work.

```
speedruns
    /bin
        /scrape
        /normalize
        /serve
    .api
        .types
        .scrape
        .normalize
    .data
        .types
        .validators
        .base
    .utils
    .server
        /views
```

## Legal

### Disclosure

This project is not associated with or endorsed by speedrun.com.

### Code License

Copyright Jeremy Banks released under the MIT License.

### Content License

See <https://www.speedrun.com/legal> for details. All data is from speedrun.com
contributors, and is used and distributed under the Creative Commons
Attribution-NonCommercial 4.0 International license.
