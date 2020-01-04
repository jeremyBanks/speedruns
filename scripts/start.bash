#!/bin/bash
set -veuxo pipefail

# source it to keep the rust server alive out here
source ./scripts/generate.bash

next dev
