import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import { NextPage } from "next";
import { useRouter } from "next/router";
import React from "react";

import Link from "next/link";
import AutoColor from "../../pages-lib/auto-color";
import Duration from "../../pages-lib/duration";
import * as schema from "../../pages-lib/schema";
import styles from "../../pages-lib/styles.module.scss";
import { withApollo } from "../../pages-lib/with-apollo";

const GamePage: NextPage = () => {
  const router = useRouter();

  const { loading, error, data } = useQuery<schema.GetGamePage>(GetGamePage, {
    variables: { slug: router.query.game },
  });

  if (!data) {
    return <>{loading ? "loading..." : JSON.stringify(error)}</>;
  }

  const game = data.game;

  if (!game) {
    return <>game not found</>;
  }

  return (
    <section className={styles.gamePage}>
      <h1>
        <Link href={`/${game.slug}`}>
          <a>{game.name}</a>
        </Link>
      </h1>

      {game.categories.map(category => (
        <div key={category.id} id={`/category/${category.id}`}>
          <h2>
            <a href={`#/category/${category.id}`}>{category.name}</a>
          </h2>

          <h3>Record Progression</h3>

          <table className={styles.progression}>
            <thead>
              <th className={styles.rank}>Rank</th>
              <th className={styles.player}>Player</th>
              <th className={styles.time}>Time (RTA)</th>
              <th className={styles.improvement}>Progress</th>
              <th className={styles.date}>Date</th>
            </thead>
            <tbody>
              <tr data-rank="1">
                <td className={styles.rank}>1</td>
                <td className={styles.player}>
                  <AutoColor>ZPR</AutoColor>
                </td>
                <td className={styles.time}>1m 31s</td>
                <td className={styles.improvement}>2s</td>
                <td className={styles.date}>
                  <AutoColor>2018-12Dec-18</AutoColor>
                </td>
              </tr>
              <tr data-rank="obsolete">
                <td className={styles.rank}>-</td>
                <td className={styles.player}>
                  <AutoColor>ZPR</AutoColor>
                </td>
                <td className={styles.time}>1m 32s</td>
                <td className={styles.improvement}>0.842s</td>
                <td className={styles.date}>
                  <AutoColor>2018-12Dec-12</AutoColor>
                </td>
              </tr>
              <tr data-rank="2">
                <td className={styles.rank}>2</td>
                <td className={styles.player}>
                  <AutoColor>Banks</AutoColor>
                </td>
                <td className={styles.time}>1m 31s</td>
                <td className={styles.improvement}>1s</td>
                <td className={styles.date}>
                  <AutoColor>2017-06Feb-02</AutoColor>
                </td>
              </tr>
              <tr data-rank="5">
                <td className={styles.rank}>5</td>
                <td className={styles.player}>
                  <AutoColor>Fralor</AutoColor>
                </td>
                <td className={styles.time}>1m 31s</td>
                <td className={styles.improvement}>8s</td>
                <td className={styles.date}>
                  <AutoColor>2016-03Mar-01</AutoColor>
                </td>
              </tr>
            </tbody>
          </table>

          <h3>Leaderboard</h3>

          <table className={styles.leaderboard}>
            <thead>
              <tr>
                <th className={styles.rank}>Rank</th>
                <th className={styles.player}>Player</th>
                <th className={styles.time}>Time (RTA)</th>
                <th className={styles.date}>Date</th>
              </tr>
            </thead>
            <tbody>
              {category.leaderboard.map(ranked => {
                return (
                  <tr key={ranked.run.id} data-rank={ranked.tiedRank}>
                    <td className={styles.rank}>{ranked.tiedRank}</td>
                    <td className={styles.player}>
                      <AutoColor>
                        {ranked.run.players.map(p => p.name).join(" & ")}
                      </AutoColor>
                    </td>
                    <td className={styles.time}>
                      <a
                        href={`https://www.speedrun.com/${game.id}/run/${ranked.run.id}`}
                      >
                        <Duration ms={ranked.timeMs} />
                      </a>
                    </td>
                    <td className={styles.date}>
                      <AutoColor>
                        {String(
                          (ranked.run.date &&
                            new Date(ranked.run.date * 1000)
                              .toISOString()
                              .slice(0, "YYYY-MM-DD".length)) ||
                            "",
                        )}
                      </AutoColor>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      ))}

      <h2>Individual Levels</h2>

      <h3>Record Progression</h3>

      <table className={styles.progression}>
        <thead>
          <th className={styles.level}>Level</th>
          <th className={styles.rank}>Rank</th>
          <th className={styles.player}>Player</th>
          <th className={styles.time}>
            Time (RTA) /
            <br />
            Sum Time
          </th>
          <th className={styles.improvement}>Progress</th>
          <th className={styles.date}>Date</th>
        </thead>
        <tbody>
          <tr data-rank="1">
            <td className={styles.level}>
              <AutoColor>Orc 01: And So On</AutoColor>
            </td>
            <td className={styles.rank}>1</td>
            <td className={styles.player}>
              <AutoColor>ZPR</AutoColor>
            </td>
            <td className={styles.time}>
              1m 31s /<br />
              1h 2m 32s
            </td>
            <td className={styles.improvement}>2s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-18</AutoColor>
            </td>
          </tr>
          <tr data-rank="1">
            <td className={styles.level}>
              <AutoColor>Orc 02: They Let On</AutoColor>
            </td>
            <td className={styles.rank}>1</td>
            <td className={styles.player}>
              <AutoColor>ZPR</AutoColor>
            </td>
            <td className={styles.time}>
              1m 32s /<br />
              1h 2m 32s
            </td>
            <td className={styles.improvement}>0.842s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-18</AutoColor>
            </td>
          </tr>
          <tr data-rank="2">
            <td className={styles.level}>
              <AutoColor>Orc 01: And So On</AutoColor>
            </td>
            <td className={styles.rank}>2</td>
            <td className={styles.player}>
              <AutoColor>Banks</AutoColor>
            </td>
            <td className={styles.time}>
              1m 31s /<br />
              1h 2m 32s
            </td>
            <td className={styles.improvement}>1s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-12</AutoColor>
            </td>
          </tr>
          <tr data-rank="1">
            <td className={styles.level}>
              <AutoColor>Orc 04: To The One</AutoColor>
            </td>
            <td className={styles.rank}>1</td>
            <td className={styles.player}>
              <AutoColor>Fralor</AutoColor>
            </td>
            <td className={styles.time}>
              1m 31s /<br />
              1h 2m 32s
            </td>
            <td className={styles.improvement}>8s</td>
            <td className={styles.date}>
              <AutoColor>2018-11Nov-12</AutoColor>
            </td>
          </tr>
        </tbody>
      </table>

      <h3>Leaderboards</h3>

      {game.levels.map(level => (
        <div key={level.id} id={`/level/${level.id}`}>
          <h4>
            <a href={`#/level/${level.id}`}>{level.name}</a>
          </h4>

          <table className={styles.leaderboard}>
            <thead>
              <tr>
                <th className={styles.rank}>Rank</th>
                <th className={styles.player}>Player</th>
                <th className={styles.time}>Time (RTA)</th>
                <th className={styles.date}>Date</th>
              </tr>
            </thead>
            <tbody>
              {level.leaderboard.map(ranked => {
                return (
                  <tr key={ranked.run.id} data-rank={ranked.tiedRank}>
                    <td className={styles.rank}>{ranked.tiedRank}</td>
                    <td className={styles.player}>
                      <AutoColor>
                        {ranked.run.players.map(p => p.name).join(" & ")}
                      </AutoColor>
                    </td>
                    <td className={styles.time}>
                      <a
                        href={`https://www.speedrun.com/${game.id}/run/${ranked.run.id}`}
                      >
                        <Duration ms={ranked.timeMs} />
                      </a>
                    </td>
                    <td className={styles.date}>
                      <AutoColor>
                        {String(
                          (ranked.run.date &&
                            new Date(ranked.run.date * 1000)
                              .toISOString()
                              .slice(0, "YYYY-MM-DD".length)) ||
                            "",
                        )}
                      </AutoColor>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      ))}
    </section>
  );
};

export default withApollo(GamePage);

const GameLeaderboardRun = gql`
  fragment GameLeaderboardRun on LeaderboardRun {
    rank
    timeMs
    isTied
    tiedRank
    run {
      id
      srcId
      category {
        id
        srcId
      }
      level {
        id
        srcId
      }
      date
      players {
        name
        isGuest
        user {
          id
          srcId
          slug
        }
      }
    }
  }
`;

const GetGamePage = gql`
  ${GameLeaderboardRun}

  query GetGamePage($slug: String!) {
    game: game(slug: $slug) {
      id
      srcId
      slug
      name
      categories {
        id
        srcId
        name
        leaderboard {
          ...GameLeaderboardRun
        }
      }
      levels {
        id
        srcId
        name
        leaderboard(categorySlug: "mission") {
          ...GameLeaderboardRun
        }
      }
    }
  }
`;
