#!/bin/bash
set -veuxo pipefail

prettier --write "**/*.js" "**/*.ts" "**/*.tsx" "**/*.md" "**/*.json" "**/*.graphql"

cargo fix --workspace --allow-dirty --allow-staged -Z unstable-options --clippy

cargo fmt --all
