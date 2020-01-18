import React from "react";

import Link from "next/link";
import styles from "./styles.module.scss";

export const HomeContent: React.FC = () => (
  <div className={styles.home}>
    <ul>
      <li>
        <Link href="/wc2">
          <a>/wc2</a>
        </Link>{" "}
        WarCraft II: Tides of Darkness
      </li>
      <li>
        <Link href="/wc2btdp">
          <a>/wc2btdp</a>
        </Link>{" "}
        WarCraft II: Beyond the Dark Portal
      </li>
      <li>
        <Link href="/wc2btdp">
          <a>/sc1</a>
        </Link>{" "}
        StarCraft
      </li>
      <li>
        <Link href="/wc2btdp">
          <a>/scbw</a>
        </Link>{" "}
        StarCraft: Brood War
      </li>
    </ul>
  </div>
);

export default HomeContent;
