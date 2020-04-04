import space from "color-space";
import sha256 from "fast-sha256";

import styles from "~/components/styles.module.scss";

const AutoColor: React.FC<{ children?: string }> = ({ children }) => {
  const seed = "2";

  const bytes = Array.from(
    sha256(
      Uint8Array.from(
        Array.from(seed + String(children)).map((c) => c.charCodeAt(0)),
      ),
    ),
  );

  const randoms = bytes.map((n) => n / 255.0);

  const [r, g, b] = space.lab
    .rgb([
      5 + 25 * randoms[0],
      -100 + 200 * randoms[1],
      -100 + 200 * randoms[2],
    ])
    .map(Math.floor);

  const [sr, sg, sb] = space.lab
    .rgb([95 + 5 * randoms[3], -20 + 40 * randoms[4], -20 + 40 * randoms[5]])
    .map(Math.floor);

  return (
    <span
      className={styles.colored}
      style={{
        background: `rgb(${sr},${sg},${sb})`,
        color: `rgb(${r},${g},${b}`,
      }}
    >
      {children}
    </span>
  );
};

export default AutoColor;
