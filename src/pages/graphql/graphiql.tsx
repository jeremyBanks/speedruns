import fetch from "isomorphic-unfetch";
import { NextPage } from "next";
import { useEffect, useState } from "react";
import { GRAPHQL_ENDPOINT } from "../../pages-lib/with-apollo";

const GraphiQLPage: NextPage = () => {
  const [GraphiQL, setGraphiQL] = useState<
    typeof import("graphiql").GraphiQL
  >();

  useEffect(() => {
    import("graphiql").then(({ GraphiQL }) => setGraphiQL(() => GraphiQL));
  }, []);

  if (typeof window === "undefined") {
    return <div>javascript required</div>;
  } else {
    if (!GraphiQL) {
      return <div>loading</div>;
    } else {
      return (
        <GraphiQL
          fetcher={(query: unknown) =>
            fetch(GRAPHQL_ENDPOINT, {
              method: "post",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify({ query }),
            }).then(response => response.json())
          }
        />
      );
    }
  }
};

export default GraphiQLPage;
