#!/bin/bash
set -euxo pipefail

cargo build --release --workspace

cargo run --release serve &
trap "kill $!" EXIT

sleep 16

yarn next build
yarn next export
