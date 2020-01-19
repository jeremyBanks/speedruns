#!/bin/bash
set -veuxo pipefail

if [ ! -f data/normalized/runs.jsonl ]; then
    echo "WARNING: no data found, so loading minimal test/fixture data"
    git checkout 48dc6d44639549ff9ebf70faba4a39956031f4fb data/normalized/{categories,games,levels,runs,users}.jsonl
    git reset HEAD data/normalized/{categories,games,levels,runs,users}.jsonl
fi

cargo build --release --bin serve

cargo run --release --bin serve &
trap "kill $!" EXIT

sleep 16

apollo client:codegen --target typescript --outputFlat src/pages-lib/schema.ts
apollo client:download-schema public/graphql/schema.json
graphql-docs-gen http://localhost:3001/graphql public/graphql/schema.html
apollo client:download-schema public/graphql/schema.apollo.graphql
cp -f node_modules/graphql-voyager/dist/voyager.worker.js public/graphql/voyager.worker.js
cp -f node_modules/graphql-voyager/dist/voyager.css public/graphql/voyager.css
