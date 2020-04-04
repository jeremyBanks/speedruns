#!/bin/bash
set -euo pipefail

cargo build --workspace

cargo test --workspace

cargo run serve &
TO_KILL="$!"
trap "kill $TO_KILL" EXIT

# something about the above is causing a signal to be sent that interrupts
# the first sleep call, so we add a second as a hacky workaround.
sleep 16
sleep 128

yarn dev &
TO_KILL="$! ${TO_KILL}"
trap "kill $TO_KILL" EXIT

sleep 4
sleep 32

yarn jest ${JEST_FLAGS:-}
