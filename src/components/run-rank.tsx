import styles from "~/components/styles.module.scss";

const RunRank: React.FC<{ rank?: number | null }> = ({ rank }) => {
  if (rank) {
    return <>{rank}</>;
  } else {
    return (
      <span
        className={styles.obsolete}
        title="obsolete: this run is no longer on the leaderboard because the player has replaced it with a better one"
      >
        n/a
      </span>
    );
  }
};

export default RunRank;
