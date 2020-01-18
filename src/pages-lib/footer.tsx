import React from "react";

import styles from "./styles.module.scss";

export const Footer: React.FC = () => (
  <footer className={styles.footer}>
    <p>
      <strong>
        The information on this site may be outdated or incomplete!
      </strong>{" "}
      The software is not entirely robust, stable, or consistent. Refer to
      speedrun.com for certainty.
    </p>

    <p>
      <div className="legal">
        This site is not affiliated with or endorsed by{" "}
        <a href="https://www.speedrun.com/about">speedrun.com</a>. This site is
        non-commercial and uses content from speedrun.com under{" "}
        <a href="https://creativecommons.org/licenses/by-nc/4.0/">
          the CC BY-NC license
        </a>
        .{" "}
        <a href="https://github.com/jeremyBanks/speedruns">
          This site's source code
        </a>{" "}
        is released under{" "}
        <a href="https://choosealicense.com/licenses/mit/">the MIT license</a>.
      </div>
    </p>
  </footer>
);

export default Footer;
