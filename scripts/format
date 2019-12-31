#!/bin/bash
set -veuxo pipefail

prettier --write "**/*.js" "**/*.ts" "**/*.tsx" "**/*.md" "**/*.json" "**/*.graphql"

cargo fmt

cargo fix --allow-dirty --allow-staged -Z unstable-options --clippy
