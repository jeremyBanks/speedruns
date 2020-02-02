#!/bin/bash
set -euxo pipefail

"$(dirname $0)/generate.bash"

cargo run serve &
trap "kill $!" EXIT

next dev
