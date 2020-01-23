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
import Head from "next/head";

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
    <section className={styles.gamePage} id={game.id} data-id={game.id}>
      <Head>
        <title>{game.name}</title>
      </Head>

      <h1>
        <Link href={`/[game]?slug=${game.slug}`} as={`/${game.slug}`}>
          <a>{game.name}</a>
        </Link>
      </h1>

      {game.gameCategories.map(category => (
        <div key={category.id} id={`${category.id}`} data-id={`${category.id}`}>
          <h2>
            <a href={`#${category.id}`}>{category.name}</a>
          </h2>

          <h3>Record Progression</h3>

          <table className={styles.progression}>
            <thead>
              <tr>
                <th className={styles.rank}>Rank</th>
                <th className={styles.player}>Player</th>
                <th className={styles.time}>Time (RTA)</th>
                <th className={styles.progress}>Progress</th>
                <th className={styles.date}>Date</th>
              </tr>
            </thead>
            <tbody>
              {category.progression.map(progress => (
                <tr
                  key={progress.run.id}
                  data-id={progress.run.id}
                  data-rank={progress.leaderboardRun?.tiedRank ?? "obsolete"}
                >
                  <td className={styles.rank}>
                    {progress.leaderboardRun?.tiedRank ?? "obsolete"}
                  </td>
                  <td className={styles.player}>
                    <AutoColor>
                      {progress.run.players.map(p => p.name).join(" & ")}
                    </AutoColor>
                  </td>
                  <td className={styles.time}>
                    <a
                      href={`https://www.speedrun.com/${game.srcSlug}/run/${progress.run.srcId}`}
                    >
                      <Duration ms={progress.run.timeMs} />
                    </a>
                  </td>
                  <td className={styles.progress}>
                    <Duration ms={progress.progressMs} />
                  </td>
                  <td className={styles.date}>
                    <AutoColor>
                      {String(
                        (progress.run.date &&
                          new Date(progress.run.date * 1000)
                            .toISOString()
                            .slice(0, "YYYY-MM-DD".length)) ||
                          "",
                      )}
                    </AutoColor>
                  </td>
                </tr>
              ))}
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
                  <tr
                    key={ranked.run.id}
                    data-id={ranked.run.id}
                    data-rank={ranked.tiedRank}
                  >
                    <td className={styles.rank}>{ranked.tiedRank}</td>
                    <td className={styles.player}>
                      <AutoColor>
                        {ranked.run.players.map(p => p.name).join(" & ")}
                      </AutoColor>
                    </td>
                    <td className={styles.time}>
                      <a
                        href={`https://www.speedrun.com/${game.srcSlug}/run/${ranked.run.srcId}`}
                      >
                        <Duration ms={ranked.run.timeMs} />
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
          <tr>
            <th className={styles.level}>Level</th>
            <th className={styles.rank}>Rank</th>
            <th className={styles.player}>Player</th>
            <th className={styles.time}>
              Time (RTA) /
              <br />
              Sum Time
            </th>
            <th className={styles.progress}>Progress</th>
            <th className={styles.date}>Date</th>
          </tr>
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
            <td className={styles.progress}>2s</td>
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
            <td className={styles.progress}>0.842s</td>
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
            <td className={styles.progress}>1s</td>
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
            <td className={styles.progress}>8s</td>
            <td className={styles.date}>
              <AutoColor>2018-11Nov-12</AutoColor>
            </td>
          </tr>
        </tbody>
      </table>

      <h3>Leaderboards</h3>

      {game.levels.map(level => (
        <div key={level.id} id={level.id} data-id={level.id}>
          <h4>
            <a href={`#${level.id}`}>{level.name}</a>
          </h4>
        </div>
      ))}
    </section>
  );
};

export default withApollo(GamePage);

const GameRun = gql`
  fragment GameRun on Run {
    id
    srcId
    timeMs
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
`;

const GameLeaderboardRun = gql`
  ${GameRun}

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
  ${GameRun}
  ${GameLeaderboardRun}

  query GetGamePage($slug: String!) {
    game: game(slug: $slug) {
      id
      srcId
      slug
      srcSlug
      name
      gameCategories {
        id
        srcId
        slug
        srcSlug
        name
        leaderboard {
          ...GameLeaderboardRun
        }
        progression {
          progressMs
          run {
            ...GameRun
          }
          leaderboardRun {
            ...GameLeaderboardRun
          }
        }
      }
      levels {
        id
        srcId
        slug
        srcSlug
        name
      }
    }
  }
`;
