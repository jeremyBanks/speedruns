import fetch from "isomorphic-unfetch";
import { NextPage } from "next";
import { useState, useEffect } from "react";

const GraphQLDocsPage: NextPage = () => {
  const [GraphQLDocs, setGraphQLDocs] = useState<
    typeof import("graphql-docs").GraphQLDocs
  >();

  useEffect(() => {
    import("graphql-docs").then(({ GraphQLDocs }) =>
      setGraphQLDocs(() => GraphQLDocs)
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
            fetch("http://localhost:3001/", {
              method: "post",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify({ query })
            }).then(response => response.json())
          }
        />
      );
    }
  }
};

export default GraphQLDocsPage;
