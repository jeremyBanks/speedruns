#!/bin/bash
set -euo pipefail

cargo build --workspace

cargo test --workspace

cargo run serve &
trap "kill $!" EXIT
yarn dev &
trap "kill $!" EXIT

sleep 16

yarn jest ${JEST_FLAGS:-}
