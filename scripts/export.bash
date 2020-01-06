#!/bin/bash
set -veuxo pipefail

cargo build --bin serve

cargo run --bin serve &
trap "kill $!" EXIT

sleep 2

yarn next build
yarn next export

graphql-docs-gen http://localhost:3001/ public/graphql/schema.html
