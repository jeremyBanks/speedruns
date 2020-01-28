import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import { NextPage } from "next";
import Link from "next/link";
import { useRouter } from "next/router";
import Head from "next/head";
import React from "react";

import AutoColor from "~/components/auto-color";
import RunDate from "~/components/run-date";
import RunDuration from "~/components/run-duration";
import RunPlayers from "~/components/run-players";
import ProgressionTable from "~/components/progression-table";
import LeaderboardTable from "~/components/leaderboard-table";
import * as schema from "~/components/schema";
import styles from "~/components/styles.module.scss";
import { withApollo, DEBUG } from "~/components/hooks/with-apollo";

const GamePage: NextPage = () => {
  const router = useRouter();

  const { loading, error, data } = useQuery<schema.GetGamePage>(GetGamePage, {
    variables: { slug: router.query.game },
    fetchPolicy: DEBUG ? "no-cache" : "cache-and-network",
    pollInterval: DEBUG ? 4000 : undefined,
  });

  if (!data) {
    return <>{loading ? "loading..." : JSON.stringify(error)}</>;
  }

  const game = data.game;

  if (!game) {
    return <>game not found</>;
  }

  return (
    <section className={styles.gamePage} id={game.id}>
      <Head>
        <title>{game.name}</title>
      </Head>

      <h2>
        <Link href={`/[game]?slug=${game.slug}`} as={`/${game.slug}`}>
          <a>{game.name}</a>
        </Link>
      </h2>

      {game.gameCategories.map(category => (
        <section key={category.id} id={`${category.id}`}>
          <h3>
            <a href={`#${category.id}`}>{category.name}</a>
          </h3>

          <h4>Progress</h4>

          <ProgressionTable runs={category.progression} />

          <h4>Leaderboard</h4>

          <LeaderboardTable runs={category.leaderboard} />
        </section>
      ))}

      <h3 id="il">
        <a href="#il">Individual Levels</a>
      </h3>

      <h4>Progress</h4>

      {game.levelCategories.map(levelCategory => (
        <ProgressionTable
          runs={levelCategory.progression}
          showLevels={true}
          showSums={true}
        />
      ))}

      {game.levels.map(level => (
        <section key={level.id} id={level.id}>
          <h3>
            <a href={`#${level.id}`}>{level.name}</a>
          </h3>

          <h4>Progress</h4>

          <ProgressionTable runs={level.progression} />

          <h4>Leaderboard</h4>

          <LeaderboardTable runs={level.leaderboard} />
        </section>
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
    game: game(slug: $slug) {
      id
      srcId
      slug
      srcSlug
      name
      gameCategories {
        id
        srcId
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
      levelCategories {
        id
        srcId
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
    }
  }
`;
