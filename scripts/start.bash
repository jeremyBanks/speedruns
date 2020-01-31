#!/bin/bash
set -euxo pipefail

if [ ! -f data/normalized/runs.jsonl ]; then
    echo "WARNING: no data found, so loading minimal test/fixture data"
    cp data/fixture/{categories,games,levels,runs,users}.jsonl data/normalized/
fi

cargo build --release --bin serve

cargo run --release --bin serve &
trap "kill $!" EXIT

next dev
