import { useQuery } from "@apollo/react-hooks";
import gql from "graphql-tag";
import { NextPage } from "next";
import { useRouter } from "next/router";
import React from "react";
import SyntaxHighlighter from "react-syntax-highlighter";
import { docco } from "react-syntax-highlighter/dist/esm/styles/hljs";

import * as schema from "../../pages-lib/schema";
import styles from "../../pages-lib/styles.module.scss";
import { withApollo } from "../../pages-lib/with-apollo";

const NodePage: NextPage = () => {
  const router = useRouter();

  const { loading, error, data } = useQuery<schema.GetNodePage>(GetNodePage, {
    variables: { id: router.query.node },
  });

  if (!data) {
    return <>{loading ? "loading..." : JSON.stringify(error)}</>;
  }

  return (
    <div className={styles.nodePage}>
      <SyntaxHighlighter language="json">
        {JSON.stringify(data, null, 4)}
      </SyntaxHighlighter>
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
      }
    }
  }
`;
