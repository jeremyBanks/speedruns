#!/bin/bash
set -euxo pipefail

if [ ! -f data/normalized/runs.jsonl ]; then
    echo "WARNING: no data found, so loading minimal test/fixture data"
    cp data/fixture/{categories,games,levels,runs,users}.jsonl data/normalized/
fi

cargo build --release

cargo run --release serve &
trap "kill $!" EXIT

next dev
