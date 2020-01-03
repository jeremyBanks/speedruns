import React from "react";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

import * as graphql from "./graphql";
import styles from "./my-games.module.scss";

export const MyGamesPage: React.FC = () => {
  const { loading, error, data } = useQuery<graphql.GetMyGames>(GetMyGames);

  if (!data || loading) {
    return <pre>loading...</pre>;
  } else if (error) {
    return <pre>error: {JSON.stringify(error, null, 2)}</pre>;
  } else {
    return <MyGames data={data} />;
  }
};

const GamePane: React.FC<{ game: graphql.MyGameDetails }> = ({ game }) => (
  <>
    <h2>Full Game</h2>

    <ol>
      {game.leaderboard.map(rank => (
        <li value={rank.tiedRank}>
          {rank.timeMs} {rank.run.id}
        </li>
      ))}
    </ol>

    <h2>Levels</h2>

    <ul>
      {game.runs.map(run => (
        <li>
          Run {run.id} in {run.category.id} {run.category.slug}{" "}
          {run.category.name}{" "}
          {run.level && (
            <>
              {run.level.id} {run.level.slug} {run.level.name}
            </>
          )}
        </li>
      ))}
    </ul>
  </>
);

export const MyGames: React.FC<{ data: graphql.GetMyGames }> = ({ data }) => {
  let games: graphql.MyGameDetails[] = [data.war2, data.war2x];

  return (
    <div className={styles.myGames}>
      <h1>WarCraft II Speedruns</h1>

      <div className={styles.games}>
        <section className={styles.war2}>
          <h1>Tides of Darkness</h1>
          <GamePane game={data.war2} />
        </section>
        <section className={styles.war2x}>
          <h1>Beyond the Dark Portal</h1>
          <GamePane game={data.war2x} />
        </section>
      </div>
    </div>
  );
};

const MyRankedRun = gql`
  fragment MyRankedRun on RankedRun {
    rank
    tiedRank
    isTied
    timeMs
    run {
      id
    }
  }
`;

const MyGameDetails = gql`
  fragment MyGameDetails on Game {
    id
    name
    slug
    leaderboard(categorySlug: "all-campaigns") {
      ...MyRankedRun
    }
    runs {
      id
      category {
        id
        slug
        name
      }
      level {
        id
        slug
        name
        #        leaderboard {
        #          ...MyRankedRun
        #        }
      }
    }
  }

  ${MyRankedRun}
`;

const GetMyGames = gql`
  query GetMyGames {
    war2: game(slug: "wc2") {
      ...MyGameDetails
    }
    war2x: game(slug: "wc2btdp") {
      ...MyGameDetails
    }
  }

  ${MyGameDetails}
`;
