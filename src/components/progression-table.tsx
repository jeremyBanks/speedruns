import styles from "~/components/styles.module.scss";
import * as schema from "~/components/schema";
import RunDate from "~/components/run-date";
import RunDuration from "~/components/run-duration";
import RunPlayers from "~/components/run-players";
import AutoColor from "~/components/auto-color";

const ProgressionTable: React.FC<{
  runs: schema.GetGamePage_game_gameCategories_progression[];
  showLevels?: boolean;
  showSums?: boolean;
  showCategories?: boolean;
}> = ({
  runs,
  showLevels = false,
  showSums = false,
  showCategories = false,
}) => (
  <table className={styles.progression}>
    <thead>
      <tr>
        {showLevels ? <th className={styles.level}>Level</th> : null}
        {showCategories ? <th className={styles.category}>Category</th> : null}
        <th className={styles.date}>Date</th>
        <th className={styles.progress}>Progress</th>
        <th className={styles.time}>Time (RTA)</th>
        <th className={styles.player}>Player</th>
        <th className={styles.rank}>Rank</th>
      </tr>
    </thead>
    <tbody>
      {runs.map(progress => (
        <tr
          key={progress.run.id}
          data-rank={progress.leaderboardRun?.rank ?? "-"}
        >
          {showLevels ? (
            <td className={styles.level}>
              <AutoColor>{progress.run.level?.name}</AutoColor>
            </td>
          ) : null}
          {showCategories ? (
            <td className={styles.level}>
              <AutoColor>{progress.run.category?.name}</AutoColor>
            </td>
          ) : null}
          <td className={styles.date}>
            <RunDate date={progress.run.date} />
          </td>
          <td className={styles.progress}>
            <RunDuration ms={progress.progressMs} />
          </td>
          <td className={styles.time}>
            <RunDuration ms={progress.run.timeMs} />
          </td>
          <td className={styles.player}>
            <RunPlayers players={progress.run.players} />
          </td>
          <td className={styles.rank}>
            {progress.leaderboardRun?.rank ?? "-"}
          </td>
        </tr>
      ))}
    </tbody>
  </table>
);

export default ProgressionTable;
