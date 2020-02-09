#!/bin/bash
set -euxo pipefail

if [ ! -f data/imported/runs.jsonl ]; then
    echo "WARNING: no data found, so loading minimal test/fixture data"
    cp data/fixture/{categories,games,levels,runs,users}.jsonl data/imported/
fi

cargo build --release --workspace

cargo run --release serve &
trap "kill $!" EXIT

sleep 16

yarn next build
yarn next export
