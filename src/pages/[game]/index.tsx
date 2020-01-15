import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import { NextPage } from "next";
import { useRouter } from "next/router";
import React from "react";

import AutoColor from "../../pages-lib/auto-color";
import * as schema from "../../pages-lib/schema";
import styles from "../../pages-lib/styles.module.scss";
import { withApollo } from "../../pages-lib/with-apollo";

const GamePage: NextPage = () => {
  const router = useRouter();

  const { loading, error, data } = useQuery<schema.GetGamePage>(GetGamePage, {
    variables: { slug: router.query.game },
  });

  if (!data) {
    return <>${loading ? "loading" : JSON.stringify(error)}</>;
  }

  const game = data.game;

  if (!game) {
    return <>game not found</>;
  }

  return (
    <section className={styles.gamePage}>
      <h1>{game.name}</h1>

      {game.categories.map(category => (
        <>
          <h2>Full Game (Category: {category.name})</h2>

          <h3>Leaderboard</h3>

          <table className={styles.leaderboard}>
            <thead>
              <th className={styles.rank}>Rank</th>
              <th className={styles.player}>Player</th>
              <th className={styles.time}>Time (RTA)</th>
              <th className={styles.date}>Date</th>
            </thead>
            <tbody>
              {category.leaderboard.map(ranked => {
                return (
                  <tr data-rank={ranked.tiedRank}>
                    <td className={styles.rank}>{ranked.tiedRank}</td>
                    <td className={styles.player}>
                      <AutoColor>
                        {ranked.run.players.map(p => p.name).join(" & ")}
                      </AutoColor>
                    </td>
                    <td className={styles.time}>{ranked.timeMs}</td>
                    <td className={styles.date}>
                      <AutoColor>{String(ranked.run.date)}</AutoColor>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </>
      ))}
    </section>
  );
};

export default withApollo(GamePage);

const GetGamePage = gql`
  query GetGamePage($slug: String!) {
    game: game(slug: $slug) {
      id
      slug
      name
      categories {
        id
        name

        leaderboard {
          rank
          timeMs
          isTied
          tiedRank
          run {
            id
            category {
              id
            }
            level {
              id
            }
            date
            players {
              name
              isGuest
              user {
                id
                slug
              }
            }
          }
        }
      }
      levels {
        id
        name
      }
    }
  }
`;
