import { NextPage } from "next";
import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import Link from "next/link";
import React, { useState, useMemo, useRef, useEffect } from "react";

import styles from "~/components/styles.module.scss";
import * as schema from "~/components/schema";
import { GRAPHQL_ENDPOINT, withApollo } from "~/components/hooks/with-apollo";
import { useDebounced } from "~/components/hooks/use-debounced";
import { version as frontendVersion } from "~/../package.json";
import useNprogress from "~/components/hooks/use-nprogress";
import LoadingBlock from "~/components/loading-block";

const searchSuggestions = [
  "Burnout",
  "WarCraft",
  "Pokemon",
  "Portal",
  "Metroid",
  "Celeste",
  "Super Mario World",
  "Spyro",
  "Mario Kart",
  "Shovel Knight",
  "Resident Evil",
  "Zelda",
  "Final Fantasy",
  "Dark Souls",
  "Quake",
];

export const HomePage: NextPage<{}> = () => {
  const home = useQuery<schema.GetHomeStats>(GetHomeStats);
  const gameIndex = useQuery<schema.GetGameIndex>(GetGameIndex, {
    ssr: false,
  });

  const [searchSuggestion, setSearchSuggestion] = useState(
    searchSuggestions[0],
  );

  const [targetName, setTargetName] = useState<string>(searchSuggestion);
  const debouncedTargetName = useDebounced(targetName, 250);
  const debouncedTargetNameOrSuggestion =
    debouncedTargetName || searchSuggestion;

  useEffect(() => {
    while (true) {
      const index = Math.floor(Math.random() * searchSuggestions.length);
      if (searchSuggestion !== searchSuggestions[index]) {
        setSearchSuggestion(searchSuggestions[index]);
        break;
      }
    }
  }, [debouncedTargetName]);

  const [targetGames, orError] = useMemo(() => {
    if (!gameIndex?.data) {
      return [null, "loading..."];
    }

    const slugify = (s: string) => {
      return s.toLowerCase().replace(/[^a-z0-9+]/g, "");
    };

    const name = slugify(debouncedTargetNameOrSuggestion);

    const matches = gameIndex.data.games
      .filter(
        (game: schema.GetGameIndex_games) =>
          slugify(game.name).includes(name) ||
          slugify(game.srcSlug).includes(name),
      )
      .sort((a: schema.GetGameIndex_games, b: schema.GetGameIndex_games) => {
        const aExact = slugify(a.srcSlug) === name || slugify(a.name) === name;
        const bExact = slugify(b.srcSlug) === name || slugify(b.name) === name;
        const aPrefix = name.startsWith(slugify(a.srcSlug));
        const bPrefix = name.startsWith(slugify(b.srcSlug));

        if (aExact && !bExact) return -1;
        else if (bExact && !aExact) return +1;
        else if (aPrefix && !bPrefix) return -1;
        else if (bPrefix && !aPrefix) return +1;
        else if (a.srcSlug.length < b.srcSlug.length) return -1;
        else if (a.srcSlug.length > b.srcSlug.length) return +1;
        else if (a.name.length < b.name.length) return -1;
        else if (a.name.length > b.name.length) return +1;
        else if (a.srcSlug < b.srcSlug) return -1;
        else if (a.srcSlug > b.srcSlug) return +1;
        else return 0;
      });

    if (matches.length === 0) {
      return [null, "no matches"];
    } else {
      return [matches.slice(0, 16), null];
    }
  }, [gameIndex?.data, debouncedTargetNameOrSuggestion]);

  const input = useRef<HTMLInputElement | null>(null);

  useEffect(() => {
    setTimeout(() => {
      const element = input.current;
      if (!element) {
        return;
      }

      element.focus();
    }, 0);
  }, []);

  useNprogress(home.loading);
  useNprogress(gameIndex.loading);

  const backendVersion = home?.data?.stats?.version;
  const backendVersionLink =
    !backendVersion || backendVersion?.endsWith("-dev")
      ? "https://github.com/jeremyBanks/speedruns/"
      : `https://crates.io/crates/speedruns/${backendVersion}`;
  const frontendVersionLink =
    !frontendVersion || frontendVersion?.endsWith("-dev")
      ? "https://github.com/jeremyBanks/speedruns/"
      : `https://www.npmjs.com/package/speedruns/v/${frontendVersion}`;

  return (
    <section className={styles.home}>
      <p>
        an unofficial mirror of{" "}
        <a href="https://www.speedrun.com">speedrun.com</a>
      </p>

      {gameIndex?.error || home?.error ? (
        <pre>{JSON.stringify([gameIndex?.error, home?.error], null, 2)}</pre>
      ) : null}

      <h2>Games</h2>

      <form
        onSubmit={event => {
          event.preventDefault();
          // HACK: sue me
          (event.target as any)
            .closest("section")
            .querySelector("ul a")
            ?.click();
        }}
      >
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
            ref={input}
            placeholder={debouncedTargetNameOrSuggestion}
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

      {gameIndex?.data ? (
        <>
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
              <p style={{ paddingLeft: "2em" }}>
                <i>
                  <b>Search failed</b>: {orError}
                </i>
              </p>
            </>
          )}
        </>
      ) : (
        <LoadingBlock />
      )}

      <h2>Internals</h2>

      {home?.data ? (
        <>
          <ul>
            <li>
              backend:{" "}
              <a href={backendVersionLink}>
                <code>{backendVersion}</code>
              </a>
            </li>
            <li>
              frontend:{" "}
              <a href={frontendVersionLink}>
                <code>{frontendVersion}</code>
              </a>
            </li>
          </ul>

          <h3>Stats</h3>

          <ul>
            <li>
              updated:{" "}
              {new Date(home.data.stats.lastUpdated)
                .toISOString()
                .slice(0, "YYYY-MM-DD".length)}
            </li>
            <li>
              games: <code>{home.data.stats.games}</code>
            </li>
            <li>
              runs: <code>{home.data.stats.runs}</code>
            </li>
          </ul>
        </>
      ) : null}

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
      version
    }
  }
`;

const GetGameIndex = gql`
  query GetGameIndex {
    games {
      name
      srcSlug
    }
  }
`;
