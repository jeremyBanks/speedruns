#!/bin/bash
set -veuxo pipefail

cargo build --bin serve

cargo run --bin serve &
trap "kill $!" EXIT

sleep 4

# committed files
apollo client:codegen --target typescript --outputFlat src/pages-lib/schema.ts
apollo client:download-schema public/graphql/schema.json

# ignored files
graphql-docs-gen http://localhost:3001/graphql public/graphql/schema.html
apollo client:download-schema public/graphql/schema.apollo.graphql
cp -f node_modules/graphql-voyager/dist/voyager.worker.js public/graphql/voyager.worker.js
cp -f node_modules/graphql-voyager/dist/voyager.css public/graphql/voyager.css
