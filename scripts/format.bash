#!/bin/bash
set -veuxo pipefail

prettier --write '**/*.{config.js,ts,tsx,md,json,graphql,gql,css,sass,scss,html}' --ignore-path '.gitignore'

cargo fmt --all
