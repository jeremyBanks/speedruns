#!/bin/bash
set -euxo pipefail

prettier --prose-wrap="always" --write '**/*.{config.js,ts,tsx,md,json,graphql,gql,css,sass,scss,html}' --ignore-path '.gitignore'

cargo fmt --all
