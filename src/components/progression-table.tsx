import styles from "~/components/styles.module.scss";
import * as schema from "~/components/schema";
import RunDate from "~/components/run-date";
import RunDuration from "~/components/run-duration";
import RunPlayers from "~/components/run-players";

const ProgressionTable: React.FC<{
  runs: schema.GetGamePage_game_gameCategories_progression[];
}> = ({ runs }) => (
  <table className={styles.progression}>
    <thead>
      <tr>
        <th className={styles.rank}>Rank</th>
        <th className={styles.player}>Player</th>
        <th className={styles.time}>Time (RTA)</th>
        <th className={styles.progress}>Progress</th>
        <th className={styles.date}>Date</th>
      </tr>
    </thead>
    <tbody>
      {runs.map(progress => (
        <tr
          key={progress.run.id}
          data-id={progress.run.id}
          data-rank={progress.leaderboardRun?.rank ?? "-"}
        >
          <td className={styles.rank}>
            {progress.leaderboardRun?.rank ?? "-"}
          </td>
          <td className={styles.player}>
            <RunPlayers players={progress.run.players} />
          </td>
          <td className={styles.time}>
            <RunDuration ms={progress.run.timeMs} />
          </td>
          <td className={styles.progress}>
            <RunDuration ms={progress.progressMs} />
          </td>
          <td className={styles.date}>
            <RunDate date={progress.run.date} />
          </td>
        </tr>
      ))}
    </tbody>
  </table>
);

export default ProgressionTable;
