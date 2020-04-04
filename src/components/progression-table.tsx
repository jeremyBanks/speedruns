import styles from "~/components/styles.module.scss";
import * as schema from "~/components/schema";
import RunDate from "~/components/run-date";
import RunDuration from "~/components/run-duration";
import RunPlayers from "~/components/run-players";
import RunRank from "~/components/run-rank";
import AutoColor from "~/components/auto-color";
import Link from "next/link";
import { FaYoutube } from "react-icons/fa";
import RunLinks from "./run-links";

const ProgressionTable: React.FC<{
  runs: schema.GetGamePage_game_gameCategories_progression[];
  showLevels?: boolean;
  showSums?: boolean;
  showCategories?: boolean;
  game: { timingMethod: string; slug: string };
}> = ({
  runs,
  showLevels = false,
  showSums = false,
  showCategories = false,
  game,
}) => (
  <table className={styles.progression}>
    <thead>
      <tr>
        {showLevels ? <th className={styles.level}>Level</th> : null}
        {showCategories ? <th className={styles.category}>Category</th> : null}
        <th className={styles.date}>Date</th>
        <th className={styles.links}>Links</th>
        <th className={styles.progress}>Progress</th>
        <th className={styles.time}>Time ({game.timingMethod})</th>
        <th className={styles.player}>Player</th>
        <th className={styles.rank}>Rank</th>
      </tr>
    </thead>
    <tbody>
      {runs.length ? (
        runs.map((progress) => (
          <tr
            key={progress.run.id}
            data-rank={progress.leaderboardRun?.rank ?? "obsolete"}
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
              <Link
                href="/[game]/run/[runSrcId]"
                as={`/${game.slug}/run/${progress.run.srcId}`}
              >
                <a>
                  <RunDate date={progress.run.date} />
                </a>
              </Link>
            </td>
            <td className={styles.links}>
              <RunLinks run={progress.run} />
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
              <RunRank rank={progress.leaderboardRun?.rank} />
            </td>
          </tr>
        ))
      ) : (
        <tr className={styles.empty}>
          <td colSpan={5}>no runs</td>
        </tr>
      )}
    </tbody>
  </table>
);

export default ProgressionTable;
