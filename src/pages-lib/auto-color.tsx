import space from "color-space";
import sha256 from "fast-sha256";

import styles from "./styles.module.scss";

const AutoColor: React.FC<{ children: string }> = ({ children }) => {
  const seed = "1";

  const bytes = Array.from(
    sha256(
      Uint8Array.from(
        Array.from(seed + String(children)).map(c => c.charCodeAt(0)),
      ),
    ),
  );

  const randoms = bytes.map(n => n / 255.0);

  const [r, g, b] = space.lab.rgb([
    5 + 20 * randoms[0],
    -100 + 200 * randoms[1],
    -100 + 200 * randoms[2],
  ]);

  const [sr, sg, sb] = space.lab.rgb([
    80 + 20 * randoms[3],
    -100 + 200 * randoms[4],
    -100 + 200 * randoms[5],
  ]);

  return (
    <span
      className={styles.colored}
      style={{
        background: `rgba(${sr}, ${sg}, ${sb}, 0.25)`,
        color: `rgb(${r}, ${g}, ${b}`,
      }}
    >
      {children}
    </span>
  );
};

export default AutoColor;
