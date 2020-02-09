#!/bin/bash
set -euxo pipefail

cargo build --workspace

cargo run serve --no-data &
trap "kill $!" EXIT

sleep 16

apollo client:codegen --target typescript --outputFlat src/components/schema.ts
get-graphql-schema http://localhost:3001/graphql --json > public/graphql/schema.json
get-graphql-schema http://localhost:3001/graphql > public/graphql/schema.graphql
cp -f node_modules/graphql-voyager/dist/voyager.worker.js public/graphql/voyager.worker.js
cp -f node_modules/graphql-voyager/dist/voyager.css public/graphql/voyager.css
