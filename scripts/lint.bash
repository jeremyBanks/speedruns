#!/bin/bash
set -veuxo pipefail

cargo clippy --workspace -- -Dwarnings;

tslint --project .;
