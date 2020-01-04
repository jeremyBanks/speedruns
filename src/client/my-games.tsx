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
      {game.leaderboard.map(rankedRun => (
        <RunLi rankedRun={rankedRun} />
      ))}
    </ol>

    <h2>Levels</h2>

    {game.levels
      .sort((a, b) => {
        if (a.name < b.name) {
          return -1;
        } else if (a.name > b.name) {
          return +1;
        } else {
          return 0;
        }
      })
      .map(level => (
        <div>
          <h3>{level.name}</h3>

          <ol>
            {level.leaderboard.map(rankedRun => (
              <RunLi rankedRun={rankedRun} />
            ))}
          </ol>
        </div>
      ))}
  </>
);

const RunLi: React.FC<{ rankedRun: graphql.MyRankedRun }> = ({ rankedRun }) => {
  const date = rankedRun.run.date;

  return (
    <li value={rankedRun.tiedRank}>
      <Duration ms={rankedRun.timeMs} />
      {date && new Date(date * 1000).toISOString()}
    </li>
  );
};

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
