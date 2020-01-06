import React from "react";
import gql from "graphql-tag";
import Link from "next/link";

import * as schema from "./schema";
import styles from "./styles.module.scss";
import { Duration } from "./duration";
import useQueryWithStatus from "./use-query-with-status";

export const HomeContent: React.FC = () => {
  const result = useQueryWithStatus<schema.GetHome>(GetHome);

  if (result.data) {
    return <Home data={result.data} />;
  } else {
    return result.status;
  }
};

export default HomeContent;

const GamePane: React.FC<{ game: schema.HomeDetails }> = ({ game }) => (
  <>
    <h2>Full Campaign</h2>

    <ol>
      {game.leaderboard.map(rankedRun => (
        <RunLi key={rankedRun.run.id} game={game} rankedRun={rankedRun} />
      ))}
    </ol>

    <h2>Individual Levels</h2>

    {game.levels
      .sort((a, b) => {
        if (a.name < b.name) {
          return -1;
        } else if (a.name > b.name) {
          return +1;
        } else {
          return 0;
        }
      })
      .map(level => (
        <div key={level.id}>
          <h3>{level.name}</h3>

          <ol>
            {level.leaderboard
              .map(rankedRun => (
                <RunLi
                  key={rankedRun.run.id}
                  game={game}
                  rankedRun={rankedRun}
                />
              ))
              .slice(0, 3)}
          </ol>
        </div>
      ))}
  </>
);

const RunLi: React.FC<{
  rankedRun: schema.HomeRankedRun;
  game: schema.HomeDetails;
}> = ({ rankedRun, game }) => {
  const date = rankedRun.run.date;

  return (
    <li value={rankedRun.tiedRank}>
      <Link
        href="/[game]/run/[run]"
        as={`/${game.slug}/run/${rankedRun.run.id}`}
      >
        <a>
          <Duration ms={rankedRun.timeMs} />
          {" by "}
          <span>
            {rankedRun.run.players.map(player => player.name).join(" and ")}
          </span>
          {" on "}
          <span>
            {date &&
              new Date(date * 1000).toISOString().slice(0, "YYYY-MM-DD".length)}
          </span>
        </a>
      </Link>
    </li>
  );
};

export const Home: React.FC<{ data: schema.GetHome }> = ({ data }) => {
  return (
    <div className={styles.home}>
      <p>
        WARNING: this is currently using a test dataset; the leadboards are not
        accurate or complete.
      </p>

      <p>
        This site compares personal and world record speed run progress over
        time.
      </p>

      <h1>WarCraft II Speedruns</h1>

      <div className={styles.games}>
        <section className={styles.war2}>
          <h1>
            <Link href="/[game]" as={`/${data.war2!.slug}/`}>
              <a>Tides of Darkness</a>
            </Link>
          </h1>
          <GamePane game={data.war2!} />
        </section>

        <section className={styles.war2x}>
          <h1>
            <Link href="/[game]" as={`/${data.war2x!.slug}/`}>
              <a>Beyond the Dark Portal</a>
            </Link>
          </h1>
          <GamePane game={data.war2x!} />
        </section>
      </div>
    </div>
  );
};

const HomeRankedRun = gql`
  fragment HomeRankedRun on RankedRun {
    rank
    tiedRank
    isTied
    timeMs
    run {
      id
      date
      players {
        name
        isGuest
        user {
          id
          slug
        }
      }
    }
  }
`;

const HomeDetails = gql`
  ${HomeRankedRun}

  fragment HomeDetails on Game {
    id
    name
    slug
    leaderboard(category: "all-campaigns") {
      ...HomeRankedRun
    }
    levels {
      id
      slug
      name
      leaderboard(category: "mission") {
        ...HomeRankedRun
      }
    }
  }
`;

const GetHome = gql`
  ${HomeDetails}

  query GetHome {
    war2: game(slug: "wc2") {
      ...HomeDetails
    }
    war2x: game(slug: "wc2btdp") {
      ...HomeDetails
    }
  }
`;
