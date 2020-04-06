#!/bin/bash
set -euo pipefail

cargo build --workspace

cargo test --workspace

cargo build --workspace --release

cargo test --workspace

cargo run --release serve &
TO_KILL="$!"

# something about the above is causing a signal to be sent that interrupts
# the first sleep call, so we add a second as a hacky workaround.
sleep 16
sleep 16
sleep 16
sleep 16
sleep 16
sleep 16
sleep 16
sleep 16

yarn dev &
TO_KILL="$! ${TO_KILL}"
trap "kill $TO_KILL" EXIT

sleep 16
sleep 16
sleep 16
sleep 16
sleep 16
sleep 16
sleep 16
sleep 16

yarn jest ${JEST_FLAGS:-}
