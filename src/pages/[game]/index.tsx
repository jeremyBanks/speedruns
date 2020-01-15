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

      <h2>Full Game (Category: All Campaigns)</h2>

      <h3>Leaderboard</h3>

      <table className={styles.leaderboard}>
        <thead>
          <th className={styles.rank}>Rank</th>
          <th className={styles.player}>Player</th>
          <th className={styles.time}>Time (RTA)</th>
          <th className={styles.date}>Date</th>
        </thead>
        <tbody>
          <tr data-rank="1">
            <td className={styles.rank}>1</td>
            <td className={styles.player}>
              <AutoColor>ZPR</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-18</AutoColor>
            </td>
          </tr>
          <tr data-rank="2">
            <td className={styles.rank}>2</td>
            <td className={styles.player}>
              <AutoColor>Banks</AutoColor>
            </td>
            <td className={styles.time}>1m 32s</td>
            <td className={styles.date}>
              <AutoColor>2018-16Dec-18</AutoColor>
            </td>
          </tr>
          <tr data-rank="3">
            <td className={styles.rank}>3</td>
            <td className={styles.player}>
              <AutoColor>GreenMixTape</AutoColor>
            </td>
            <td className={styles.time}>2m 0s</td>
            <td className={styles.date}>
              <AutoColor>2018-10Dec-18</AutoColor>
            </td>
          </tr>
          <tr data-rank="4">
            <td className={styles.rank}>4</td>
            <td className={styles.player}>
              <AutoColor>KarmikKoala</AutoColor>
            </td>
            <td className={styles.time}>2m 31s</td>
            <td className={styles.date}>
              <AutoColor>2014-12Dec-01</AutoColor>
            </td>
          </tr>
          <tr data-rank="5">
            <td className={styles.rank}>5</td>
            <td className={styles.player}>
              <AutoColor>Fralor</AutoColor>
            </td>
            <td className={styles.time}>4m 31s</td>
            <td className={styles.date}>
              <AutoColor>2018-02Feb-18</AutoColor>
            </td>
          </tr>
        </tbody>
      </table>
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
    }
  }
`;
