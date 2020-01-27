import fetch from "isomorphic-unfetch";
import { NextPage } from "next";
import { useEffect, useState } from "react";
import NoSSR from "react-no-ssr";

import styles from "~/components/styles.module.scss";
import { GRAPHQL_ENDPOINT } from "~/components/hooks/with-apollo";

const GraphiQLPage: NextPage<{}> = () => (
  <NoSSR onSSR={<>Loading...</>}>
    <div className={styles.graphiqlFrame}>
      <GraphiQL />
    </div>
  </NoSSR>
);

const GraphiQL: React.FC<{}> = () => {
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
        <div className={styles.graphiqlFrame}>
          <GraphiQL
            fetcher={(query: unknown) =>
              fetch(GRAPHQL_ENDPOINT, {
                body: JSON.stringify(query),
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

export default GraphiQLPage;
