import Link from "next/link";
import React from "react";

import styles from "~/components/styles.module.scss";

// XXX: This link doesn't seem to be handled by Next, but that might be
// this issue which is apparently fixed in the next release:
// https://github.com/zeit/next-plugins/issues/282

const Header: React.FC = () => {
  return (
    <header className={styles.header}>
      <h1 className={styles.text}>
        <span className={styles.inner}>
          <Link href="/">
            <a>
              <img src="/finch.png" width="13" /> <b>speedruns.ca</b>
            </a>
          </Link>
          :{" "}
          <span className={styles.subtitle}>
            an unofficial mirror of{" "}
            <a href="https://www.speedrun.com/">speedrun.com</a>
          </span>
        </span>
      </h1>
    </header>
  );
};

export default Header;
