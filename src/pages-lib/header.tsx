import React from "react";

import styles from "./styles.module.scss";

export const Header: React.FC = () => {
  return (
    <header className={styles.header}>
      <h1 className={styles.text}>
        <span className={styles.inner}>
          <img src="/contest.png" />
          <a href="/">speedrun.ca</a>
        </span>
      </h1>
    </header>
  );
};

export default Header;
