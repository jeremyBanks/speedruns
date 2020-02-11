import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import { NextPage } from "next";
import Head from "next/head";
import { useRouter } from "next/router";
import React from "react";

import * as schema from "~/components/schema";
import styles from "~/components/styles.module.scss";
import { withApollo } from "~/components/hooks/with-apollo";

const NodePage: NextPage = () => {
  const router = useRouter();

  const { loading, error, data } = useQuery<schema.GetNodePage>(GetNodePage, {
    variables: { id: router.query.node },
    // The use of fragments in this query requires special handling to be
    // cached and we haven't done that, so let's disable it for now.
    // https://www.apollographql.com/docs/react/data/fragments/
    fetchPolicy: "no-cache",
  });

  if (!data) {
    return <>{error ? JSON.stringify(error) : ""}</>;
  }

  const node = data.node && { ...data.node };

  if (!node) {
    return (
      <div className={styles.nodePage}>
        <pre>null</pre>
      </div>
    );
  }

  console.log(node);
  const typename = node.__typename;
  const id = node.id;
  delete node.__typename;
  delete node.id;

  return (
    <div className={styles.nodePage}>
      <Head>
        <title>{id}</title>
      </Head>

      <pre>
        <span className={styles.typeName}>{typename}</span>(id:{" "}
        {JSON.stringify(id)}) {JSON.stringify(node, null, 4)}
      </pre>
    </div>
  );
};

export default withApollo(NodePage);

const GetNodePage = gql`
  query GetNodePage($id: ID!) {
    node: node(id: $id) {
      __typename
      id
      ... on Game {
        srcId
        srcSlug
        name
        slug
      }
      ... on User {
        srcId
        srcSlug
        slug
      }
      ... on Level {
        srcId
        srcSlug
        name
        slug
      }
      ... on Category {
        srcId
        srcSlug
        name
        slug
      }
      ... on Run {
        srcId
        date
        category {
          id
          srcId
        }
        level {
          id
          srcId
        }
        timeMs
      }
    }
  }
`;
