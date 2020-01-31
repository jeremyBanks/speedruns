#!/bin/bash
set -euxo pipefail

if [ ! -f data/normalized/runs.jsonl ]; then
    echo "WARNING: no data found, so loading minimal test/fixture data"
    git checkout 48dc6d44639549ff9ebf70faba4a39956031f4fb data/normalized/{categories,games,levels,runs,users}.jsonl
    git reset HEAD data/normalized/{categories,games,levels,runs,users}.jsonl
fi

cargo build --release --bin serve

cargo run --release --bin serve &
trap "kill $!" EXIT

next dev
