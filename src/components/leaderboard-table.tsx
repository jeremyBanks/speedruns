import styles from "~/components/styles.module.scss";
import * as schema from "~/components/schema";
import RunDate from "~/components/run-date";
import RunDuration from "~/components/run-duration";
import RunPlayers from "~/components/run-players";
import RunRank from "~/components/run-rank";

import Link from "next/link";

const LeaderboardTable: React.FC<{
  runs: schema.GetGamePage_game_gameCategories_leaderboard[];
  game: { timingMethod: string; srcSlug: string };
}> = ({ runs, game }) => (
  <table className={styles.leaderboard}>
    <thead>
      <tr>
        <th className={styles.rank}>Rank</th>
        <th className={styles.player}>Player</th>
        <th className={styles.time}>Time ({game.timingMethod})</th>
        <th className={styles.date}>Date</th>
      </tr>
    </thead>
    <tbody>
      {runs.map(leaderboardRun => (
        <tr
          key={leaderboardRun.run.id}
          data-rank={leaderboardRun?.tiedRank ?? "obsolete"}
        >
          <td className={styles.rank}>
            <RunRank rank={leaderboardRun?.tiedRank} />
          </td>
          <td className={styles.player}>
            <RunPlayers players={leaderboardRun.run.players} />
          </td>
          <td className={styles.time}>
            <RunDuration ms={leaderboardRun.run.timeMs} />
          </td>
          <td className={styles.date}>
            <Link
              href="/[game]/run/[runSrcId]"
              as={`/${game.srcSlug}/run/${leaderboardRun.run.srcId}`}
            >
              <a>
                <RunDate date={leaderboardRun.run.date} />
              </a>
            </Link>
          </td>
        </tr>
      ))}
    </tbody>
  </table>
);

export default LeaderboardTable;
