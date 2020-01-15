import { NextPage } from "next";
import React from "react";

import AutoColor from "../../pages-lib/auto-color";
import styles from "../../pages-lib/styles.module.scss";

const MockupPage: NextPage = () => {
  return (
    <section className={styles.gamePage}>
      <p>
        mockup. I'd still like the ability to include personal progression in
        the world record view but I guess that can come later. or what about
        adding personal rank to every progression row?{" "}
      </p>

      <h1>My Favourite Game</h1>

      <h2>Full Game (Category: All Campaigns)</h2>

      <h3>Record Progression</h3>

      <table className={styles.progression}>
        <thead>
          <th className={styles.rank}>Rank</th>
          <th className={styles.player}>Player</th>
          <th className={styles.time}>Time (RTA)</th>
          <th className={styles.improvement}>Progress</th>
          <th className={styles.date}>Date</th>
        </thead>
        <tbody>
          <tr data-rank="1">
            <td className={styles.rank}>1</td>
            <td className={styles.player}>
              <AutoColor>ZPR</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.improvement}>2s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-18</AutoColor>
            </td>
          </tr>
          <tr data-rank="obsolete">
            <td className={styles.rank}>-</td>
            <td className={styles.player}>
              <AutoColor>ZPR</AutoColor>
            </td>
            <td className={styles.time}>1m 32s</td>
            <td className={styles.improvement}>0.842s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-12</AutoColor>
            </td>
          </tr>
          <tr data-rank="2">
            <td className={styles.rank}>2</td>
            <td className={styles.player}>
              <AutoColor>Banks</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.improvement}>1s</td>
            <td className={styles.date}>
              <AutoColor>2017-06Feb-02</AutoColor>
            </td>
          </tr>
          <tr data-rank="5">
            <td className={styles.rank}>5</td>
            <td className={styles.player}>
              <AutoColor>Fralor</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.improvement}>8s</td>
            <td className={styles.date}>
              <AutoColor>2016-03Mar-01</AutoColor>
            </td>
          </tr>
        </tbody>
      </table>
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

      <h2>Individual Levels (Category: Mission)</h2>

      <h3>Record Progression</h3>

      <table className={styles.progression}>
        <thead>
          <th className={styles.level}>Level</th>
          <th className={styles.rank}>Rank</th>
          <th className={styles.player}>Player</th>
          <th className={styles.time}>
            Time (RTA) /
            <br />
            Sum Time
          </th>
          <th className={styles.improvement}>Progress</th>
          <th className={styles.date}>Date</th>
        </thead>
        <tbody>
          <tr data-rank="1">
            <td className={styles.level}>
              <AutoColor>Orc 01: And So On</AutoColor>
            </td>
            <td className={styles.rank}>1</td>
            <td className={styles.player}>
              <AutoColor>ZPR</AutoColor>
            </td>
            <td className={styles.time}>
              1m 31s /<br />
              1h 2m 32s
            </td>
            <td className={styles.improvement}>2s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-18</AutoColor>
            </td>
          </tr>
          <tr data-rank="1">
            <td className={styles.level}>
              <AutoColor>Orc 02: They Let On</AutoColor>
            </td>
            <td className={styles.rank}>1</td>
            <td className={styles.player}>
              <AutoColor>ZPR</AutoColor>
            </td>
            <td className={styles.time}>
              1m 32s /<br />
              1h 2m 32s
            </td>
            <td className={styles.improvement}>0.842s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-18</AutoColor>
            </td>
          </tr>
          <tr data-rank="2">
            <td className={styles.level}>
              <AutoColor>Orc 01: And So On</AutoColor>
            </td>
            <td className={styles.rank}>2</td>
            <td className={styles.player}>
              <AutoColor>Banks</AutoColor>
            </td>
            <td className={styles.time}>
              1m 31s /<br />
              1h 2m 32s
            </td>
            <td className={styles.improvement}>1s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-12</AutoColor>
            </td>
          </tr>
          <tr data-rank="1">
            <td className={styles.level}>
              <AutoColor>Orc 04: To The One</AutoColor>
            </td>
            <td className={styles.rank}>1</td>
            <td className={styles.player}>
              <AutoColor>Fralor</AutoColor>
            </td>
            <td className={styles.time}>
              1m 31s /<br />
              1h 2m 32s
            </td>
            <td className={styles.improvement}>8s</td>
            <td className={styles.date}>
              <AutoColor>2018-11Nov-12</AutoColor>
            </td>
          </tr>
        </tbody>
      </table>

      <h3>Leaderboards</h3>

      <h4>Orc 01: Blorrowmere</h4>

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
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-20</AutoColor>
            </td>
          </tr>
          <tr data-rank="3">
            <td className={styles.rank}>3</td>
            <td className={styles.player}>
              <AutoColor>GreenMixTape</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-11</AutoColor>
            </td>
          </tr>
          <tr data-rank="4">
            <td className={styles.rank}>4</td>
            <td className={styles.player}>
              <AutoColor>KarmikKoala</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-14</AutoColor>
            </td>
          </tr>
          <tr data-rank="5">
            <td className={styles.rank}>5</td>
            <td className={styles.player}>
              <AutoColor>Fralor</AutoColor>
            </td>
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-09</AutoColor>
            </td>
          </tr>
        </tbody>
      </table>

      <h4>Orc 02: Bloblosphere</h4>

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
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-20</AutoColor>
            </td>
          </tr>
        </tbody>
      </table>

      <h4>Orc 03: The End</h4>
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
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-20</AutoColor>
            </td>
          </tr>
        </tbody>
      </table>

      <h4>Human 01: Or Is It</h4>
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
            <td className={styles.time}>1m 31s</td>
            <td className={styles.date}>
              <AutoColor>2018-12Dec-20</AutoColor>
            </td>
          </tr>
        </tbody>
      </table>
    </section>
  );
};

export default MockupPage;
