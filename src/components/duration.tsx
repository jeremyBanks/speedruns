import React from "react";

import styles from "~/components/styles.module.scss";

export const Duration: React.FC<{ ms: number }> = ({ ms }) => {
  const msPart = ms % 1000;
  const s = Math.floor(ms / 1000);
  const sPart = s % 60;
  const m = Math.floor(s / 60);
  const mPart = m % 60;
  const h = Math.floor(m / 60);

  const pieces = [];
  if (h > 0) {
    pieces.push(`${h}h`);
  }
  if (mPart > 0 || pieces.length) {
    pieces.push(`${String(mPart).padStart(2, "0")}m`);
  }
  pieces.push(`${String(sPart).padStart(2, "0")}`);
  if (msPart) {
    pieces.push(`.${String(msPart).padStart(3, "0")}s`);
  } else {
    pieces.push("s   ");
  }

  return <span className={styles.duration}>{pieces}</span>;
};

export default Duration;
