#!/bin/bash
set -euxo pipefail

host=172.105.11.193
cargo run --release download;
cargo run --release import;
scp -rp data/normalized/ speedrun@$host:/home/speedrun/speedruns/data/;
ssh speedrun@$host 'ls /home/speedrun/speedruns/data/normalized -la';
ssh root@$host 'systemctl restart graphql';
