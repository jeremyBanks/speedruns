import React from "react";

import Link from "next/link";
import styles from "./styles.module.scss";

export const HomeContent: React.FC = () => (
  <div className={styles.home}>
    <ul>
      <li>
        <Link href="/wc2">
          <a>/wc2</a>
        </Link>
      </li>
      <li>
        <Link href="/wc2btdp">
          <a>/wc2btdp</a>
        </Link>
      </li>
    </ul>
  </div>
);

export default HomeContent;
