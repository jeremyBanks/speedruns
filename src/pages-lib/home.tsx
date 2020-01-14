import React from "react";

import styles from "./styles.module.scss";

export const HomeContent: React.FC = () => (
  <div className={styles.home}>
    <p>
      WARNING: this is currently using a test dataset; the leadboards are not
      accurate or complete.
    </p>

    <p>
      This site compares personal and world record speed run progress over time.
    </p>
  </div>
);

export default HomeContent;
