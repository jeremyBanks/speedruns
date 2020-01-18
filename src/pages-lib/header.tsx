import Link from "next/link";
import React from "react";

import styles from "./styles.module.scss";

export const Header: React.FC = () => {
  return (
    <header className={styles.header}>
      <h1 className={styles.text}>
        <span className={styles.inner}>
          <img src="/icon.png" />{" "}
          <Link href="/">
            <a>speedrun.ca</a>
          </Link>
        </span>
      </h1>
    </header>
  );
};

export default Header;
