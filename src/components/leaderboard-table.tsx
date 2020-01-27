import styles from "~/components/styles.module.scss";
import * as schema from "~/components/schema";
import RunDate from "~/components/run-date";
import RunDuration from "~/components/run-duration";
import RunPlayers from "~/components/run-players";

const LeaderboardTable: React.FC<{
  runs: schema.GetGamePage_game_gameCategories_leaderboard[];
}> = ({ runs }) => (
  <table className={styles.leaderboard}>
    <thead>
      <tr>
        <th className={styles.rank}>Rank</th>
        <th className={styles.player}>Player</th>
        <th className={styles.time}>Time (RTA)</th>
        <th className={styles.date}>Date</th>
      </tr>
    </thead>
    <tbody>
      {runs.map(leaderboardRun => (
        <tr key={leaderboardRun.run.id} data-rank={leaderboardRun?.rank ?? "-"}>
          <td className={styles.rank}>{leaderboardRun?.rank ?? "-"}</td>
          <td className={styles.player}>
            <RunPlayers players={leaderboardRun.run.players} />
          </td>
          <td className={styles.time}>
            <RunDuration ms={leaderboardRun.run.timeMs} />
          </td>
          <td className={styles.date}>
            <RunDate date={leaderboardRun.run.date} />
          </td>
        </tr>
      ))}
    </tbody>
  </table>
);

export default LeaderboardTable;
