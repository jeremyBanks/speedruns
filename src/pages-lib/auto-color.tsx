import space from "color-space";
import sha256 from "fast-sha256";

import styles from "./styles.module.scss";

const AutoColor: React.FC<{ children: string }> = ({ children }) => {
  const seed = "2";

  const bytes = Array.from(
    sha256(
      Uint8Array.from(
        Array.from(seed + String(children)).map(c => c.charCodeAt(0)),
      ),
    ),
  );

  const randoms = bytes.map(n => n / 255.0);

  const [r, g, b] = space.lab.rgb([
    Math.floor(5 + 25 * randoms[0]),
    Math.floor(-100 + 200 * randoms[1]),
    Math.floor(-100 + 200 * randoms[2]),
  ]);

  const [sr, sg, sb] = space.lab.rgb([
    Math.floor(95 + 5 * randoms[3]),
    Math.floor(-20 + 40 * randoms[4]),
    Math.floor(-20 + 40 * randoms[5]),
  ]);

  return (
    <span
      className={styles.colored}
      style={{
        background: `rgb(${sr}, ${sg}, ${sb})`,
        color: `rgb(${r}, ${g}, ${b}`,
      }}
    >
      {children}
    </span>
  );
};

export default AutoColor;
