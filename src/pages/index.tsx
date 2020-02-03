import { NextPage } from "next";
import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import Link from "next/link";
import React, { useState, useMemo } from "react";

import styles from "~/components/styles.module.scss";
import * as schema from "~/components/schema";
import { GRAPHQL_ENDPOINT, withApollo } from "~/components/hooks/with-apollo";
import { useDebounced } from "~/components/hooks/use-debounced";

export const HomePage: NextPage<{}> = () => {
  const { loading, error, data } = useQuery<schema.GetHomeStats>(GetHomeStats);

  const [defaultSearch, _] = useState(() => {
    const options = [
      "WarCraft",
      "Celeste",
      "Super Mario World",
      "Link to the Past",
      "Burnout",
      "Spyro",
      "Mario Kart",
      "Shovel Knight",
      "Resident Evil",
      "Crash Bandicoot",
      "Final Fantasy X",
    ];
    const index = Math.floor(Math.random() * options.length);
    return options[index];
  });

  const [targetName, setTargetName] = useState<string>(defaultSearch);
  const debouncedTargetName = useDebounced(targetName, 250) || defaultSearch;
  const [targetGames, orError] = useMemo(() => {
    if (!data) {
      return [null, "loading..."];
    }

    const slugify = (s: string) => {
      return s.toLowerCase().replace(/[^a-z0-9+]/g, "");
    };

    const name = slugify(debouncedTargetName);

    const matches = data.games
      .filter(
        game =>
          slugify(game.name).includes(name) ||
          slugify(game.srcSlug).includes(name),
      )
      .sort((a, b) => {
        if (a.srcSlug < b.srcSlug) return -1;
        else if (a.srcSlug > b.srcSlug) return +1;
        else return 0;
      });

    if (matches.length > 64) {
      return [null, `too many matches (${matches.length})`];
    } else if (matches.length === 0) {
      return [null, "no matches"];
    } else {
      return [matches, null];
    }
  }, [data, debouncedTargetName]);

  if (!data) {
    return <>{loading ? "loading..." : JSON.stringify(error)}</>;
  }

  const { stats } = data;

  return (
    <section className={styles.home}>
      <p>An unofficial mirror of speedrun.com.</p>

      <h2>Games</h2>

      <form>
        <label
          style={{
            display: "flex",
            alignItems: "center",
          }}
        >
          <span
            style={{
              display: "flex",
              flex: 0,
              padding: "4px 8px",
            }}
          >
            Search:
          </span>
          <input
            placeholder={debouncedTargetName}
            onChange={e => void setTargetName(e.target.value)}
            style={{
              display: "flex",
              flex: 1,
              fontSize: "18px",
              padding: "4px 8px",
            }}
          />
        </label>
      </form>

      {targetGames ? (
        <ul>
          {targetGames.map(({ srcSlug, name }) => (
            <li key={srcSlug}>
              <Link href={`/[game]?game=${srcSlug}`} as={`/${srcSlug}`}>
                <a>
                  <code>
                    <b>/{srcSlug}</b>
                  </code>{" "}
                  {name}
                </a>
              </Link>
            </li>
          ))}
        </ul>
      ) : (
        <>
          <p>
            <b>Search failed:</b> {orError}
          </p>
        </>
      )}

      <h2>Internals</h2>

      <h3>Stats</h3>

      <ul>
        <li>Last Updated: {new Date(stats.lastUpdated).toISOString()}</li>
        <li>
          Games: <code>{stats.games}</code>
        </li>
        <li>
          Runs: <code>{stats.runs}</code>
        </li>
      </ul>

      <h3>GraphQL Schema</h3>

      <ul>
        <li>
          <Link href="/graphql/schema">
            <a>Documentation</a>
          </Link>
        </li>
        <li>
          <Link href="/graphql/voyager">
            <a>Graph Viewer</a>
          </Link>
        </li>
        <li>
          <Link href="/node/gamdtuPrEpI">
            <a>Node Inspector</a>
          </Link>
        </li>
        <li>
          <a href="/graphql/schema.graphql">SDL Definition</a>
        </li>
        <li>
          <a href="/graphql/schema.json">Introspection JSON</a>
        </li>
        <li>
          <a href={`${GRAPHQL_ENDPOINT}/../playground`}>Playground IDE</a>
        </li>
        <li>
          <a href={`${GRAPHQL_ENDPOINT}/../graphiql`}>GraphiQL IDE</a>
        </li>
        <li>
          <a href={GRAPHQL_ENDPOINT}>Endpoint</a>
        </li>
      </ul>
    </section>
  );
};

export default withApollo(HomePage);

const GetHomeStats = gql`
  query GetHomeStats {
    stats {
      lastUpdated
      runs
      games
    }

    games {
      name
      srcSlug
    }
  }
`;
