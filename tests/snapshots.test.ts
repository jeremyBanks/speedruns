import fetch from "node-fetch";

test("snapshot API", async () => {
  const response = await fetch("http://localhost:3001/graphql", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body:
      '{"operationName":"GetGamePage","variables":{"slug":"wc2"},"query":"fragment GameRun on Run {\\n  id\\n  srcId\\n  timeMs\\n  videos\\n  category {\\n    id\\n    srcId\\n    name\\n    __typename\\n  }\\n  level {\\n    id\\n    srcId\\n    slug\\n    name\\n    __typename\\n  }\\n  date\\n  players {\\n    name\\n    isGuest\\n    user {\\n      id\\n      srcId\\n      slug\\n      __typename\\n    }\\n    __typename\\n  }\\n  __typename\\n}\\n\\nfragment GameLeaderboardRun on LeaderboardRun {\\n  rank\\n  isTied\\n  tiedRank\\n  run {\\n    id\\n    srcId\\n    timeMs\\n    videos\\n    category {\\n      id\\n      srcId\\n      name\\n      __typename\\n    }\\n    level {\\n      id\\n      srcId\\n      slug\\n      name\\n      __typename\\n    }\\n    date\\n    players {\\n      name\\n      isGuest\\n      user {\\n        id\\n        srcId\\n        slug\\n        __typename\\n      }\\n      __typename\\n    }\\n    __typename\\n  }\\n  __typename\\n}\\n\\nquery GetGamePage($slug: String!) {\\n  game(slug: $slug) {\\n    id\\n    srcId\\n    slug\\n    slug\\n    name\\n    timingMethod\\n    gameCategories {\\n      id\\n      srcId\\n      slug\\n      name\\n      leaderboard(limit: 32) {\\n        ...GameLeaderboardRun\\n        __typename\\n      }\\n      progression {\\n        progressMs\\n        run {\\n          ...GameRun\\n          __typename\\n        }\\n        leaderboardRun {\\n          ...GameLeaderboardRun\\n          __typename\\n        }\\n        __typename\\n      }\\n      __typename\\n    }\\n    levelCategories {\\n      id\\n      srcId\\n      slug\\n      name\\n      leaderboard(limit: 32) {\\n        ...GameLeaderboardRun\\n        __typename\\n      }\\n      levels {\\n        level {\\n          id\\n          srcId\\n          slug\\n          name\\n          __typename\\n        }\\n        leaderboard(limit: 32) {\\n          ...GameLeaderboardRun\\n          __typename\\n        }\\n        progression {\\n          progressMs\\n          run {\\n            ...GameRun\\n            __typename\\n          }\\n          leaderboardRun {\\n            ...GameLeaderboardRun\\n            __typename\\n          }\\n          __typename\\n        }\\n        __typename\\n      }\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n"}',
  });
  expect(response.statusText).toMatchSnapshot();
  const body = await response.json();
  expect(body).toMatchSnapshot();
});

test("snapshot home page", async () => {
  const response = await fetch("http://localhost:3000/");
  expect(response.statusText).toMatchSnapshot();
  const body = await (await response.text()).replace(/\?ts=\d+"/g, '?ts=0"');
  expect(body).toMatchSnapshot();
});
