#!/bin/bash
set -veuxo pipefail

prettier --write '**/*.{config.js,ts,tsx,md,json,graphql,gql,css,sass,scss,html}' --ignore-path '.gitignore'

# XXX: this is more than formatting...
cargo fix --workspace --allow-dirty --allow-staged -Z unstable-options --clippy

cargo fmt --all

tslint --fix || true
