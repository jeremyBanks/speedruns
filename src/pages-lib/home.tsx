import React from "react";

import Link from "next/link";
import styles from "./styles.module.scss";
import { GRAPHQL_ENDPOINT } from "./with-apollo";

export const HomeContent: React.FC = () => (
  <section className={styles.home}>
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

    <h3>GraphQL Schema</h3>

    <ul>
      <li>
        <a href="/graphql/schema.html">Documentation</a>
      </li>
      <li>
        <a href="/graphql/voyager">Graph Viewer</a>
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
        <a href={GRAPHQL_ENDPOINT}>Endpoint</a>
      </li>
    </ul>
  </section>
);

export default HomeContent;
