#!/bin/bash
set -veuxo pipefail

cargo build --bin serve

cargo run --bin serve &
trap "kill $!" EXIT

sleep 2

apollo client:codegen --target typescript --outputFlat src/pages-lib/schema.ts
apollo client:download-schema public/graphql/schema.graphql
graphql-docs-gen http://localhost:3001/ public/graphql/schema.html
