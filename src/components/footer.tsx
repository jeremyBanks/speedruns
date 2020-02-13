import React from "react";

import styles from "~/components/styles.module.scss";

const Footer: React.FC = () => (
  <footer className={styles.footer}>
    <p>
      <strong>
        The information on this site may be incomplete or aggregated
        incorrectly.
      </strong>{" "}
      Refer to <a href="https://www.speedrun.com/games">speedrun.com</a> for
      certainty.
    </p>

    <p>
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
    </p>
  </footer>
);

export default Footer;
