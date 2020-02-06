# speedruns

Unofficial tools for mirroring [speedrun.com](https://www.speedrun.com/) API data. Using Rust, TypeScript, GraphQL, and React.

## Legal

### Disclosure

This project is not associated with or endorsed by speedrun.com.

### Code License

Copyright Jeremy Banks, released under [the MIT License](./LICENSE).

### Content License

See <https://www.speedrun.com/legal> for details. All data is from speedrun.com
contributors, and is used and distributed under the Creative Commons
Attribution-NonCommercial 4.0 International license.

## Development

### Environment

Install [rustup](https://rustup.rs/) to manage the Rust toolchain and [nvm](https://github.com/nvm-sh/nvm) to manage the JavaScript toolchain, following instructions on their respective pages.

Install and activate our expected version of npm using nvm, then install [the Yarn package manager](https://yarnpkg.com/):

```
nvm install .
nvm use .
npm install -g yarn@1
```

### Run Server

You can start the backend and frontend servers together using our yarn `start` script. You'll probably want to open the frontend in your browser at http://localhost:3000.

```
yarn start
```

Initially, this will load a test fixture data set only intended for development.

### Import Data

If you want to use the real data set, you'll need to import it.

(Optional) You may want to start by downloading archived copies of `{runs,users,games}.jsonl.gz` from
https://archive.org/download/speedrun.com-2020-02-01, and putting them in `data/api`, to reduce the amount you load through the API (which can take a long time).

Download any new data by running:

```
cargo run download
```

This can take a long time (potentially a full day if you're starting from scratch). This won't include changes or deletions of existing items, we assume they're unchanged. If you want to be sure that every record is up to date, you need to delete the downloaded data and start from scratch.

Validate and convert the downloaded API data into our internal format by running:

```
cargo run import
```

Any records that don't match our expected format (missing now-required fields, inconsistent timing methods, or niche options we don't support) **will be discarded**, so our leaderboards might not match speedrun.com (whose software robustly accomidates old data of varied shapes).

Restart the server to load the new data.
