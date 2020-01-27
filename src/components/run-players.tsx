import AutoColor from "~/components/auto-color";
import * as schema from "~/components/schema";

const RunPlayers: React.FC<{ players: schema.GameRun_players[] }> = ({
  players,
}) => {
  if (players?.length) {
    return <AutoColor>{players.map(p => p.name).join(" & ")}</AutoColor>;
  } else {
    return <></>;
  }
};

export default RunPlayers;
