#!/bin/bash
set -euo pipefail

echo > last-benchmark.txt

lscpu | egrep 'Model name:|CPU\(s\):|CPU MHz:' | sort -r | tee -a last-benchmark.txt

echo | tee -a last-benchmark.txt

free --human | tee -a last-benchmark.txt

echo | tee -a last-benchmark.txt


cargo build --release --bin serve
(RUST_LOG="speedruns=info,serve=info" cargo run --release --bin serve | tee -a last-benchmark.txt) &
trap "kill $!" EXIT

sleep 8

echo | tee -a last-benchmark.txt

autocannon 'http://localhost:3001/graphql' \
    --json \
    --headers "content-type=application/json" \
    --duration '8' \
    --method 'POST' \
    --body '{"operationName":"GetGamePage","variables":{"slug":"wc2"},"query":"fragment GameLeaderboardRun on LeaderboardRun {\n  rank\n  timeMs\n  isTied\n  tiedRank\n  run {\n    id\n    category {\n      id\n      __typename\n    }\n    level {\n      id\n      __typename\n    }\n    date\n    players {\n      name\n      isGuest\n      user {\n        id\n        slug\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nquery GetGamePage($slug: String!) {\n  game: game(slug: $slug) {\n    id\n    slug\n    name\n    categories {\n      id\n      name\n      leaderboard {\n        ...GameLeaderboardRun\n        __typename\n      }\n      __typename\n    }\n    levels {\n      id\n      name\n      leaderboard(categorySlug: \"mission\") {\n        ...GameLeaderboardRun\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n"}' | jqn --color=false 'pick("requests.average errors timeouts 2xx non2xx latency.average latency.min latency.max".split(" "))' | tee -a last-benchmark.txt