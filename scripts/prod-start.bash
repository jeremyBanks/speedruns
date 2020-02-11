#!/bin/bash
set -vexuo pipefail

source ~/.bashrc
source $HOME/.cargo/env
cd ~/speedruns

export PATH="$PATH:/home/speedrun/.nvm/versions/node/v13.5.0/bin/"

npm install -g --force speedruns@^0.21.6-dev.0
version="$(speedruns-frontend --version)"

curl -L -o speedruns-linux-x86_64 https://github.com/jeremyBanks/speedruns/releases/download/$version/speedruns-linux-x86_64
chmod +x speedruns-linux-x86_64

GRAPHQL_ENDPOINT=http://localhost:3001/graphql speedruns-frontend &
./speedruns-linux-x86_64 serve
