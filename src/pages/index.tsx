import { NextPage } from "next";
import Link from "next/link";
import React from "react";

import styles from "../pages-lib/styles.module.scss";
import { GRAPHQL_ENDPOINT, withApollo } from "../pages-lib/with-apollo";

export const HomePage: NextPage<{}> = () => (
  <section className={styles.home}>
    <p>An unofficial mirror of speedrun.com.</p>

    <h2>Games</h2>

    <ul>
      <li>
        <Link href="/wc2">
          <a>/wc2</a>
        </Link>{" "}
        WarCraft II: Tides of Darkness
      </li>
      <li>
        <Link href="/wc2btdp">
          <a>/wc2btdp</a>
        </Link>{" "}
        WarCraft II: Beyond the Dark Portal
      </li>
      <li>
        <Link href="/wc2btdp">
          <a>/sc1</a>
        </Link>{" "}
        StarCraft
      </li>
      <li>
        <Link href="/wc2btdp">
          <a>/scbw</a>
        </Link>{" "}
        StarCraft: Brood War
      </li>
      <li>
        <Link href="/celeste">
          <a>/celeste</a>
        </Link>{" "}
        Celeste
      </li>
    </ul>

    <h2>Internals</h2>

    <h3>Stats</h3>

    <ul>
      <li>Last Updated: never?</li>
      <li>
        Games: <code>2</code>
      </li>
      <li>
        Runs: <code>3</code>
      </li>
    </ul>

    <h3>GraphQL Schema</h3>

    <ul>
      <li>
        <a href="/graphql/schema">Documentation</a>
      </li>
      <li>
        <a href="/graphql/voyager">Graph Viewer</a>
      </li>
      <li>
        <a href="/node/G-ABtuPrEpI">Node Inspector</a>
      </li>
      <li>
        <a href="/graphql/schema.graphql">SDL for Juniper</a>
      </li>
      <li>
        <a href="/graphql/schema.apollo.graphql">SDL from Apollo</a>
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

export default withApollo(HomePage);
