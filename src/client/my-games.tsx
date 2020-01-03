import React from "react";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

import * as graphql from "./graphql";
import styles from "./my-games.module.css";

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

export const MyGames: React.FC<{ data: graphql.GetMyGames }> = ({ data }) => {
  let me = data.banks;
  let games: graphql.MyGameDetails[] = [data.war2, data.war2btdp];

  return (
    <div className={styles.content}>
      <h1>
        {me.id} {me.slug}
      </h1>

      {games.map(game => (
        <>
          <h1>
            {game.id} {game.name}
          </h1>

          <h2>Leaderboard</h2>

          <ol>
            {game.leaderboard.map(rank => (
              <li value={rank.tiedRank}>
                {rank.timeMs} {rank.run.id}
              </li>
            ))}
          </ol>

          <h2>All Runs</h2>

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
      ))}
    </div>
  );
};

const MyGameDetails = gql`
  fragment MyGameDetails on Game {
    id
    name
    slug
    leaderboard(categorySlug: "all-campaigns") {
      rank
      tiedRank
      isTied
      timeMs
      run {
        id
      }
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
      }
    }
  }
`;

const GetMyGames = gql`
  query GetMyGames {
    banks: user(slug: "banks") {
      id
      slug
    }
    war2: game(slug: "wc2") {
      ...MyGameDetails
    }
    war2btdp: game(slug: "wc2btdp") {
      ...MyGameDetails
    }
  }

  ${MyGameDetails}
`;
