import space from "color-space";
import sha256 from "fast-sha256";

const AutoColor: React.FC<{ children: string }> = ({ children }) => {
  const bytes = Array.from(
    sha256(
      Uint8Array.from(Array.from(String(children)).map(c => c.charCodeAt(0))),
    ),
  );

  const randoms = bytes.map(n => n / 255.0);

  const [r, g, b] = space.lab.rgb([
    5 + 20 * randoms[0],
    -100 + 200 * randoms[1],
    -100 + 200 * randoms[2],
  ]);

  return <span style={{ color: `rgb(${r}, ${g}, ${b}` }}>{children}</span>;
};

export default AutoColor;
