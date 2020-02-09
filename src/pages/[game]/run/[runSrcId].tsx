import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import { NextPage } from "next";
import Link from "next/link";
import { useRouter } from "next/router";
import Head from "next/head";
import React from "react";

import * as schema from "~/components/schema";
import styles from "~/components/styles.module.scss";
import { withApollo, DEBUG } from "~/components/hooks/with-apollo";
import RunDuration from "~/components/duration";
import RunPlayers from "~/components/run-players";
import RunDate from "~/components/run-date";

const RunPage: NextPage = () => {
  const router = useRouter();

  const { loading, error, data } = useQuery<schema.GetRunPage>(GetRunPage, {
    variables: {
      gameSrcSlug: router.query.game,
      runSrcId: router.query.runSrcId,
    },
    fetchPolicy: DEBUG ? "cache-and-network" : "cache-first",
  });

  if (!data) {
    return <>{loading ? "loading..." : JSON.stringify(error)}</>;
  }

  const game = data.game;
  const run = data.run;

  if (!game || !run) {
    // TODO: also verify that the run belongs to this game
    return <>run not found</>;
  }

  return (
    <section className={styles.runPage} id={game.id}>
      <Head>
        <title>{game.name}</title>
        <link
          rel="canonical"
          href={`https://www.speedrun.com/${game.srcSlug}/run/${run.srcId}`}
        />
      </Head>
      <h2>
        <Link href={`/[game]?game=${game.slug}`} as={`/${game.slug}`}>
          <a>{game.name}</a>
        </Link>
      </h2>
      <h3>
        <Link
          href={`/[game]/run/[runSrcId]?game=${game.slug}&runSrcId=${run.srcId}`}
          as={`/${game.slug}/run/${run.srcId}`}
        >
          <a>
            Run {run.srcId} on {run.level?.name ?? "full game"} (
            {run.category.name})
          </a>
        </Link>
      </h3>
      <p>
        in <RunDuration ms={run.timeMs} /> by{" "}
        <RunPlayers players={run.players} />
        on <RunDate date={run.date} />
      </p>
    </section>
  );
};

export default withApollo(RunPage);

const GetRunPage = gql`
  query GetRunPage($gameSrcSlug: String!, $runSrcId: ID!) {
    game(slug: $gameSrcSlug) {
      id
      srcId
      slug
      srcSlug
      name
    }

    run(srcId: $runSrcId) {
      id
      srcId
      timeMs
      category {
        id
        srcId
        name
      }
      level {
        id
        srcId
        srcSlug
        name
      }
      date
      players {
        name
        isGuest
        user {
          id
          srcId
          slug
        }
      }
    }
  }
`;
