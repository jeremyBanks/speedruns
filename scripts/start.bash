#!/bin/bash
set -euxo pipefail

yarn generate

yarn format

cargo build --release --workspace

cargo run --release serve &
trap "kill $!" EXIT

sleep 16

next dev
