import React from "react";

import styles from "./styles.module.scss";

export const Duration: React.FC<{ ms: number }> = ({ ms }) => {
  let ms_part = ms % 1000;
  let s = Math.floor(ms / 1000);
  let s_part = s % 60;
  let m = Math.floor(s / 60);
  let m_part = m % 60;
  let h = Math.floor(m / 60);

  let pieces = [];
  if (h > 0) {
    pieces.push(`${h}h`);
  }
  if (m_part > 0 || pieces.length) {
    pieces.push(`${String(m_part).padStart(2, "0")}m`);
  }
  pieces.push(`${String(s_part).padStart(2, "0")}`);
  if (ms_part) {
    pieces.push(`.${String(ms_part).padStart(3, "0")}s`);
  } else {
    pieces.push("s   ");
  }

  return <span className={styles.duration}>{pieces}</span>;
};
