import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import { NextPage } from "next";
import Link from "next/link";
import { useRouter } from "next/router";
import Head from "next/head";
import React from "react";

import ProgressionTable from "~/components/progression-table";
import LeaderboardTable from "~/components/leaderboard-table";
import * as schema from "~/components/schema";
import styles from "~/components/styles.module.scss";
import { withApollo, DEBUG } from "~/components/hooks/with-apollo";
import useProgressIndicator from "~/components/hooks/use-nprogress";
import LoadingBlock from "~/components/loading-block";

const GamePage: NextPage = () => {
  const router = useRouter();

  const { loading, error, data } = useQuery<schema.GetGamePage>(GetGamePage, {
    variables: { slug: router.query.game },
  });

  useProgressIndicator(loading);

  if (error || !data) {
    return <>{error ? JSON.stringify(error) : <LoadingBlock />}</>;
  }

  const game = data?.game;

  if (!game) {
    return <>game not found</>;
  }

  return (
    <section className={styles.gamePage}>
      <Head>
        <title>{game.name} Speedruns</title>
        <link rel="canonical" href={`https://www.speedrun.com/${game.slug}`} />
        <meta property="og:title" content={`${game.name} Speedruns`} />
        <meta property="og:image" content="/finch.png" />
        <meta
          property="og:description"
          content={`speedrun leaderboards and progression for ${game.name}`}
        />
      </Head>

      <h2>
        <Link href={`/[game]?game=${game.slug}`} as={`/${game.slug}`}>
          <a>{game.name}</a>
        </Link>
      </h2>

      {game.gameCategories.map(category => (
        <section key={category.id} id={`${category.id}`}>
          <h3>
            <a href={`#${category.id}`}>{category.name}</a>
          </h3>

          <h4>Progress</h4>

          <ProgressionTable runs={category.progression} game={game} />

          <h4>Leaderboard</h4>

          <LeaderboardTable runs={category.leaderboard} game={game} />
        </section>
      ))}

      {game.levelCategories.map(levelCategory => (
        <div key={levelCategory.id} id={levelCategory.id}>
          <h2>
            <a href={`#${levelCategory.id}`}>{levelCategory.name}</a>
          </h2>

          {levelCategory.levels.map(({ level, leaderboard, progression }) => (
            <section key={level.id} id={`${levelCategory.id}${level.id}`}>
              <h3>
                <a href={`#${levelCategory.id}${level.id}`}>{level.name}</a>
              </h3>

              <h4>Progress</h4>

              <ProgressionTable runs={progression} game={game} />

              <h4>Leaderboard</h4>

              <LeaderboardTable runs={leaderboard} game={game} />
            </section>
          ))}
        </div>
      ))}
    </section>
  );
};

export default withApollo(GamePage);

const GetGamePagePreview = gql`
  query GetGamePagePreview($slug: String!) {
    game(slug: $slug) {
      id
      slug
      name
    }
  }
`;

const GameRun = gql`
  fragment GameRun on Run {
    id
    srcId
    timeMs
    videos
    category {
      id
      srcId
      name
    }
    level {
      id
      srcId
      slug
      name
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
      videos
      category {
        id
        srcId
        name
      }
      level {
        id
        srcId
        slug
        name
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
    game(slug: $slug) {
      id
      srcId
      slug
      slug
      name
      timingMethod
      gameCategories {
        id
        srcId
        slug
        name
        leaderboard(limit: 32) {
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
      levelCategories {
        id
        srcId
        slug
        name
        leaderboard(limit: 32) {
          ...GameLeaderboardRun
        }
        levels {
          level {
            id
            srcId
            slug
            name
          }
          leaderboard(limit: 32) {
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
      }
    }
  }
`;
