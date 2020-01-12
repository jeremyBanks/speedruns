# speedruns

Tools for mirroring speedrun.com data.

## Usage

Assumes `rustup install .` and `nvm use .` and that you have `yarn` installed.

Optionally, download archived copies of `{runs,users,games}.jsonl.gz` from
https://archive.org/download/speedrun.com-2019-04-27 and put them in `data/api`.
If you skip this, you'll need to spend about a day download everything from the
API yourself.

Run `cargo run --release --bin scrape` to download any new data. This doesn't
include changes or deletions of existing items.

Run `cargo run --release --bin normalize` to normalize the data, including only
essential fields and records (no rejected or pending runs), and discarding
anything that fails validation or integrity checking.

Run `yarn start` to start the dev server.

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

## Legal

### Disclosure

This project is not associated with or endorsed by speedrun.com.

### Code License

Copyright Jeremy Banks, released under the MIT License.

### Content License

See <https://www.speedrun.com/legal> for details. All data is from speedrun.com
contributors, and is used and distributed under the Creative Commons
Attribution-NonCommercial 4.0 International license.
