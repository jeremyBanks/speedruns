#!/bin/bash
set -euxo pipefail

if [ ! -f data/normalized/runs.jsonl ]; then
    echo "WARNING: no data found, so loading minimal test/fixture data"
    cp data/fixture/{categories,games,levels,runs,users}.jsonl data/normalized/
fi

cargo build --release --workspace

cargo run --release serve &
trap "kill $!" EXIT

sleep 16

yarn next build
yarn next export
