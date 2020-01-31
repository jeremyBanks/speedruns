import { NextPage } from "next";
import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import Link from "next/link";
import React from "react";

import styles from "~/components/styles.module.scss";
import * as schema from "~/components/schema";
import { GRAPHQL_ENDPOINT, withApollo } from "~/components/hooks/with-apollo";

export const HomePage: NextPage<{}> = () => {
  const { loading, error, data } = useQuery<schema.GetHomeStats>(GetHomeStats);

  if (!data) {
    return <>{loading ? "loading..." : JSON.stringify(error)}</>;
  }

  const { stats } = data;

  return (
    <section className={styles.home}>
      <p>An unofficial mirror of speedrun.com.</p>

      <h2>Games</h2>

      <ul>
        <li>
          <Link href="/[game]?game=wc2" as="/wc2">
            <a>/wc2</a>
          </Link>{" "}
          WarCraft II: Tides of Darkness
        </li>
        <li>
          <Link href="/[game]?game=wc2btdp" as="/wc2btdp">
            <a>/wc2btdp</a>
          </Link>{" "}
          WarCraft II: Beyond the Dark Portal
        </li>
        <li>
          <Link href="/[game]?game=sc1" as="/sc1">
            <a>/sc1</a>
          </Link>{" "}
          StarCraft
        </li>
        <li>
          <Link href="/[game]?game=scbw" as="/scbw">
            <a>/scbw</a>
          </Link>{" "}
          StarCraft: Brood War
        </li>
        <li>
          <Link href="/[game]?game=celeste" as="/celeste">
            <a>/celeste</a>
          </Link>{" "}
          Celeste
        </li>
      </ul>

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
  }
`;
