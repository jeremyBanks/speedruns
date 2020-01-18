import fetch from "isomorphic-unfetch";
import { NextPage } from "next";
import { useEffect, useState } from "react";
import { GRAPHQL_ENDPOINT } from "../../pages-lib/with-apollo";

const GraphiQLPage: NextPage = () => {
  const [GraphiQL, setGraphiQL] = useState<typeof import("graphiql").default>();

  useEffect(() => {
    import("graphiql").then(imported => {
      setGraphiQL(() => imported.default);
    });
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
              body: JSON.stringify(query),
              headers: { "Content-Type": "application/json" },
              method: "post",
            }).then(response => response.json())
          }
        />
      );
    }
  }
};

export default GraphiQLPage;
