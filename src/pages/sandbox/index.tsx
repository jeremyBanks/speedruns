import { NextPage } from "next";
import React from "react";

import AutoColor from "../../pages-lib/auto-color";
import styles from "./styles.module.scss";

const SandboxPage: NextPage = () => {
  return (
    <section className={styles.sandbox}>
      <h1>My Favourite Game</h1>

      <h2>Full Game</h2>

      <h3>Leaderboard</h3>

      <table className={styles.leaderboard}>
        <thead>
          <th className={styles.rank}>Rank</th>
          <th className={styles.player}>Player</th>
          <th className={styles.time}>Time</th>
          <th className={styles.date}>Date</th>
        </thead>
        <tbody>
          <tr data-rank="1">
            <td className={styles.rank}>1</td>
            <td className={styles.player}>
              <AutoColor>ZPR</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>2018-12Dec-18</td>
          </tr>
          <tr data-rank="2">
            <td className={styles.rank}>2</td>
            <td className={styles.player}>
              <AutoColor>Banks</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>2018-12Dec-18</td>
          </tr>
          <tr data-rank="3">
            <td className={styles.rank}>3</td>
            <td className={styles.player}>
              <AutoColor>GreenMixTape</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>2018-12Dec-18</td>
          </tr>
          <tr data-rank="4">
            <td className={styles.rank}>4</td>
            <td className={styles.player}>
              <AutoColor>KarmikKoala</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>2018-12Dec-18</td>
          </tr>
          <tr data-rank="5">
            <td className={styles.rank}>5</td>
            <td className={styles.player}>
              <AutoColor>Fralor</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>2018-12Dec-18</td>
          </tr>
        </tbody>
      </table>

      <h3>History</h3>

      <h2>Individual Levels</h2>

      <h3>History</h3>
    </section>
  );
};

export default SandboxPage;
