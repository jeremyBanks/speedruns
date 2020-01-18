#!/bin/bash
set -veuxo pipefail

RUSTFLAGS="-D warnings" cargo build --workspace;

RUSTFLAGS="-D warnings" cargo clippy --workspace;

tslint --project .;
