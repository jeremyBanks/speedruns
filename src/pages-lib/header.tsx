import Link from "next/link";
import React from "react";

import styles from "./styles.module.scss";

// XXX: This link doesn't seem to be handled by Next, but that might be
// this issue which is apparently fixed in the next release:
// https://github.com/zeit/next-plugins/issues/282

export const Header: React.FC = () => {
  return (
    <header className={styles.header}>
      <h1 className={styles.text}>
        <span className={styles.inner}>
          <img src="/icon.png" />{" "}
          <Link href="/index" as="/">
            <a>speedruns.ca</a>
          </Link>
        </span>
      </h1>
    </header>
  );
};

export default Header;
