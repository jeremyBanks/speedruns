import AutoColor from "~/components/auto-color";

const RunDate: React.FC<{ date: number | null }> = ({ date }) => {
  if (date) {
    return (
      <AutoColor>
        {new Date(date * 1000).toISOString().slice(0, "YYYY-MM-DD".length)}
      </AutoColor>
    );
  } else {
    return <></>;
  }
};

export default RunDate;
