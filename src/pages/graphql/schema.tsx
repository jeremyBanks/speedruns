import fetch from "isomorphic-unfetch";
import { NextPage } from "next";
import { useEffect, useState } from "react";
import { GRAPHQL_ENDPOINT } from "../../pages-lib/with-apollo";

const GraphQLDocsPage: NextPage = () => {
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
        <GraphQLDocs
          fetcher={(query: unknown) =>
            fetch(`${GRAPHQL_ENDPOINT}/graphql`, {
              body: JSON.stringify({ query }),
              headers: { "Content-Type": "application/json" },
              method: "post",
            }).then(response => response.json())
          }
        />
      );
    }
  }
};

export default GraphQLDocsPage;
