#!/bin/bash
set -veuxo pipefail

cargo build --bin serve

cargo run --bin serve &
trap "kill $!" EXIT

sleep 2

apollo client:codegen --target typescript --outputFlat src/pages-lib/schema.ts
apollo client:download-schema public/graphql/schema.json

cp -f node_modules/graphql-voyager/dist/voyager.worker.js public/graphql/voyager.worker.js
cp -f node_modules/graphql-voyager/dist/voyager.css public/graphql/voyager.css
