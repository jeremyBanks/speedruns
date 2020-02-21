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

  // TODO: make this use a cached value from the home page, instead of a second query
  const { data: previewData } = useQuery<schema.GetGamePagePreview>(
    GetGamePagePreview,
    {
      variables: { slug: router.query.game },
    },
  );

  const { loading, error, data } = useQuery<schema.GetGamePage>(GetGamePage, {
    variables: { slug: router.query.game },
  });

  useProgressIndicator(loading);

  if (error || !(data || previewData)) {
    return <>{error ? JSON.stringify(error) : <LoadingBlock />}</>;
  }

  const fullGame = data?.game;
  const game = fullGame || previewData?.game;

  if (!game) {
    return <>game not found</>;
  }

  return (
    <section className={styles.gamePage}>
      <Head>
        <title>{game.name} Speedruns</title>
        <link
          rel="canonical"
          href={`https://www.speedrun.com/${game.srcSlug}`}
        />
      </Head>

      <h2>
        <Link href={`/[game]?game=${game.srcSlug}`} as={`/${game.srcSlug}`}>
          <a>{game.name}</a>
        </Link>
      </h2>

      {fullGame?.gameCategories.map(category => (
        <section key={category.id} id={`${category.id}`}>
          <h3>
            <a href={`#${category.id}`}>{category.name}</a>
          </h3>

          <h4>Progress</h4>

          <ProgressionTable runs={category.progression} game={fullGame} />

          <h4>Leaderboard</h4>

          <LeaderboardTable runs={category.leaderboard} game={fullGame} />
        </section>
      ))}

      {fullGame?.levelCategories.map(levelCategory => (
        <div key={levelCategory.id} id={levelCategory.id}>
          <h2>
            <a href={`#${levelCategory.id}`}>{levelCategory.name}</a>
          </h2>

          <h4>Progress</h4>

          <ProgressionTable
            runs={levelCategory.progression}
            showLevels={true}
            showSums={true}
            game={fullGame}
          />

          {levelCategory.levels.map(({ level, leaderboard, progression }) => (
            <section key={level.id} id={`${levelCategory.id}${level.id}`}>
              <h3>
                <a href={`#${levelCategory.id}${level.id}`}>{level.name}</a>
              </h3>

              <h4>Progress</h4>

              <ProgressionTable runs={progression} game={fullGame} />

              <h4>Leaderboard</h4>

              <LeaderboardTable runs={leaderboard} game={fullGame} />
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
      srcSlug
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
      srcSlug
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
        srcSlug
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
      srcSlug
      name
      timingMethod
      gameCategories {
        id
        srcId
        srcSlug
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
        srcSlug
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
        levels {
          level {
            id
            srcId
            srcSlug
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
