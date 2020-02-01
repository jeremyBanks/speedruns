#!/bin/bash
set -euxo pipefail

if [ ! -f data/imported/runs.jsonl ]; then
    echo "WARNING: no data found, so loading minimal test/fixture data"
    cp data/fixture/{categories,games,levels,runs,users}.jsonl data/imported/
fi

cargo build --release

cargo run --release serve &
trap "kill $!" EXIT

next dev
