#!/bin/bash
set -euxo pipefail

cargo build --release --bin serve

cargo run --release --bin serve &
trap "kill $!" EXIT

sleep 16

yarn next build
yarn next export
