test("snapshots", async done => {
  const response = await require("node-fetch")(
    "http://localhost:3001/graphql",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: `{"operationName":"GetGamePage","variables":{"slug":"celeste"},"query":"fragment GameRun on Run {
            id
          srcId
          timeMs
          category {
              id
            srcId
            name
            __typename
      }
        level {
              id
            srcId
            slug
            name
            __typename
      }
        date
        players {
              name
            isGuest
            user {
                id
              srcId
              slug
              __typename
        }
          __typename
      }
        __typename
    }
      fragment GameLeaderboardRun on LeaderboardRun {
            rank
          isTied
          tiedRank
          run {
              id
            srcId
            timeMs
            category {
                id
              srcId
              name
              __typename
        }
          level {
                id
              srcId
              slug
              name
              __typename
        }
          date
          players {
                name
              isGuest
              user {
                  id
                srcId
                slug
                __typename
          }
            __typename
        }
          __typename
      }
        __typename
    }
      query GetGamePage($slug: String!) {
            game: game(slug: $slug) {
              id
            srcId
            slug
            slug
            name
            gameCategories {
                id
              srcId
              slug
              name
              leaderboard {
                  ...GameLeaderboardRun
                __typename
          }
            progression {
                  progressMs
                run {
                    ...GameRun
                  __typename
            }
              leaderboardRun {
                    ...GameLeaderboardRun
                  __typename
            }
              __typename
          }
            __typename
        }
          levelCategories {
                id
              srcId
              slug
              name
              leaderboard {
                  ...GameLeaderboardRun
                __typename
          }
            progression {
                  progressMs
                run {
                    ...GameRun
                  __typename
            }
              leaderboardRun {
                    ...GameLeaderboardRun
                  __typename
            }
              __typename
          }
            levels {
                  level {
                    id
                  srcId
                  slug
                  name
                  __typename
            }
              leaderboard {
                    ...GameLeaderboardRun
                  __typename
            }
              progression {
                    progressMs
                  run {
                      ...GameRun
                    __typename
              }
                leaderboardRun {
                      ...GameLeaderboardRun
                    __typename
              }
                __typename
            }
              __typename
          }
            __typename
        }
          __typename
      }
    }
      "}' | jqn --color=false 'pick("requests.average errors timeouts 2xx non2xx latency.average latency.min latency.max".split(" "))`,
    },
  );
  const data = await response.json();
  expect(data).toMatchSnapshot();

  done();
});
