#!/bin/bash
set -euxo pipefail
tslint --fix --project . || echo "tslint failed"

cargo fix --workspace --allow-dirty --allow-staged
