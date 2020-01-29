#!/bin/bash
set -euo pipefail

TARGET=last-benchmark.txt

echo > "${TARGET}"

lscpu | egrep 'Model name:|CPU\(s\):|CPU MHz:' | sort -r | tee -a "${TARGET}"

echo | tee -a "${TARGET}"

free --human | tee -a "${TARGET}"

echo | tee -a "${TARGET}"


cargo build --release --bin serve
(RUST_LOG="speedruns=info,serve=info" cargo run --release --bin serve | tee -a "${TARGET}") &
trap "kill $!" EXIT

sleep 16

echo | tee -a "${TARGET}"

autocannon 'http://localhost:3001/graphql' \
    --json \
    --headers "content-type=application/json" \
    --duration '32' \
    --method 'POST' \
    --body '{"operationName":"GetGamePage","variables":{"slug":"celeste"},"query":"fragment GameRun on Run {\n  id\n  srcId\n  timeMs\n  category {\n    id\n    srcId\n    name\n    __typename\n  }\n  level {\n    id\n    srcId\n    srcSlug\n    name\n    __typename\n  }\n  date\n  players {\n    name\n    isGuest\n    user {\n      id\n      srcId\n      slug\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment GameLeaderboardRun on LeaderboardRun {\n  rank\n  isTied\n  tiedRank\n  run {\n    id\n    srcId\n    timeMs\n    category {\n      id\n      srcId\n      name\n      __typename\n    }\n    level {\n      id\n      srcId\n      srcSlug\n      name\n      __typename\n    }\n    date\n    players {\n      name\n      isGuest\n      user {\n        id\n        srcId\n        slug\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nquery GetGamePage($slug: String!) {\n  game: game(slug: $slug) {\n    id\n    srcId\n    slug\n    srcSlug\n    name\n    gameCategories {\n      id\n      srcId\n      srcSlug\n      name\n      leaderboard {\n        ...GameLeaderboardRun\n        __typename\n      }\n      progression {\n        progressMs\n        run {\n          ...GameRun\n          __typename\n        }\n        leaderboardRun {\n          ...GameLeaderboardRun\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    levelCategories {\n      id\n      srcId\n      srcSlug\n      name\n      leaderboard {\n        ...GameLeaderboardRun\n        __typename\n      }\n      progression {\n        progressMs\n        run {\n          ...GameRun\n          __typename\n        }\n        leaderboardRun {\n          ...GameLeaderboardRun\n          __typename\n        }\n        __typename\n      }\n      levels {\n        level {\n          id\n          srcId\n          srcSlug\n          name\n          __typename\n        }\n        leaderboard {\n          ...GameLeaderboardRun\n          __typename\n        }\n        progression {\n          progressMs\n          run {\n            ...GameRun\n            __typename\n          }\n          leaderboardRun {\n            ...GameLeaderboardRun\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n"}' | jqn --color=false 'pick("requests.average errors timeouts 2xx non2xx latency.average latency.min latency.max".split(" "))' | tee -a "${TARGET}"

git diff "${TARGET}";
