import fetch from "isomorphic-unfetch";
import { NextPage } from "next";
import { useEffect, useState } from "react";
import NoSSR from "react-no-ssr";

import styles from "../../components/styles.module.scss";
import { GRAPHQL_ENDPOINT } from "../../components/with-apollo";

const GraphQLDocsPage: NextPage<{}> = () => (
  <NoSSR onSSR={<>Loading...</>}>
    <div className={styles.schemaFrame}>
      <GraphQLDocs />
    </div>
  </NoSSR>
);

const GraphQLDocs: React.FC<{}> = () => {
  const [GraphQLDocs, setGraphQLDocs] = useState<
    typeof import("graphql-docs").GraphQLDocs
  >();

  useEffect(() => {
    import("graphql-docs").then(({ GraphQLDocs }) =>
      setGraphQLDocs(() => GraphQLDocs),
    );
  }, []);

  if (typeof window === "undefined") {
    return <div>javascript required</div>;
  } else {
    if (!GraphQLDocs) {
      return <div>loading</div>;
    } else {
      return (
        <div>
          <GraphQLDocs
            fetcher={(query: unknown) =>
              fetch(`${GRAPHQL_ENDPOINT}`, {
                body: JSON.stringify({ query }),
                headers: { "Content-Type": "application/json" },
                method: "post",
              }).then(response => response.json())
            }
          />
        </div>
      );
    }
  }
};

export default GraphQLDocsPage;
