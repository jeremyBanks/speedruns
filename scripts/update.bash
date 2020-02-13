#!/bin/bash
set -euxo pipefail

host=172.105.11.193
cargo run --release download;
cargo run --release import;
cargo run --release import --fixtures;
scp -rp data/imported/ speedrun@$host:/home/speedrun/speedruns/data/;
ssh speedrun@$host 'ls /home/speedrun/speedruns/data/imported -la';
ssh root@$host 'systemctl restart graphql';
