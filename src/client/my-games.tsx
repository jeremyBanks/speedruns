import React from "react";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

import * as graphql from "./graphql";
import styles from "./styles.module.scss";
import { Duration } from "./duration";

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
          <Duration ms={rank.timeMs} /> {rank.run.id}
        </li>
      ))}
    </ol>
  </>
);

export const MyGames: React.FC<{ data: graphql.GetMyGames }> = ({ data }) => {
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
      date
      players
    }
  }
`;

const MyGameDetails = gql`
  fragment MyGameDetails on Game {
    id
    name
    slug
    leaderboard(category: "all-campaigns") {
      ...MyRankedRun
    }

    levels {
      id
      slug
      name
      leaderboard(category: "mission") {
        ...MyRankedRun
      }
    }

    runs {
      id
      category {
        id
        slug
        name
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
