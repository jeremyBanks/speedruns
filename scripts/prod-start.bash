#!/bin/bash
set -vexuo pipefail;

source ~/.bashrc;
source $HOME/.cargo/env;
cd ~/speedruns;

git pull --force;

export PATH="$PATH:/home/speedrun/.nvm/versions/node/v13.5.0/bin/";

npm install -g speedruns@^0.20.20-dev.0;
cargo install speedruns --force --version '>0.20.20-dev.0';

GRAPHQL_ENDPOINT=http://localhost:3001 speedruns-frontend &;
speedruns serve
