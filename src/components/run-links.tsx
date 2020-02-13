import { FaYoutube, FaTwitch, FaLink } from "react-icons/fa";

import {
  GetGamePage_game_gameCategories_leaderboard_run,
  GetGamePage_game_gameCategories_progression_run,
} from "~/components/schema";

const RunLinks = ({
  run,
}: {
  run:
    | GetGamePage_game_gameCategories_leaderboard_run
    | GetGamePage_game_gameCategories_progression_run;
}) => (
  <>
    {run.videos.map((video, index) => (
      <a href={video} key={index}>
        {/^https?:\/\/(www\.)?(youtu\.be|youtube.com)\//i.test(video) ? (
          <FaYoutube title={video} />
        ) : /^https?:\/\/(www\.)?(twitch\.tv)\//i.test(video) ? (
          <FaTwitch title={video} />
        ) : (
          <FaLink title={video} />
        )}
      </a>
    ))}
  </>
);

export default RunLinks;
