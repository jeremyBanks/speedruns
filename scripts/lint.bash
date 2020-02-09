#!/bin/bash
set -euxo pipefail

RUSTFLAGS="-D warnings" cargo build --workspace;

RUSTFLAGS="-D warnings" cargo clippy --workspace;

tslint --project .;
