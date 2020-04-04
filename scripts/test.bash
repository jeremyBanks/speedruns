#!/bin/bash
set -euo pipefail

cargo build --workspace

cargo test --workspace

cargo run serve &
TO_KILL="$! ${TO_KILL:-}"
trap "kill $TO_KILL" EXIT

yarn dev &
TO_KILL="$! ${TO_KILL:-}"
trap "kill $TO_KILL" EXIT

sleep 16

yarn jest ${JEST_FLAGS:-}
