#!/bin/bash
set -veuxo pipefail

tslint --fix --project . || echo "tslint failed"

cargo fix --workspace --allow-dirty --allow-staged -Z unstable-options --clippy
