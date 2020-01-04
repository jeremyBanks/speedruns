#!/bin/bash
set -veuxo pipefail

prettier --write '**/*.{js,ts,tsx,md,json,graphql,gql,css,sass,scss}' --ignore-path '.gitignore'

# XXX: this is more than formatting...
cargo fix --workspace --allow-dirty --allow-staged -Z unstable-options --clippy

cargo fmt --all
