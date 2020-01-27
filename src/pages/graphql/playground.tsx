import { NextPage } from "next";
import { useEffect, useState } from "react";
import NoSSR from "react-no-ssr";
import { Provider } from "react-redux";

import styles from "~/components/styles.module.scss";
import { GRAPHQL_ENDPOINT } from "~/components/hooks/with-apollo";

const PlaygroundPage: NextPage<{}> = () => (
  <NoSSR onSSR={<>Loading...</>}>
    <div className={styles.playgroundFrame}>
      <Playground />
    </div>
  </NoSSR>
);

const Playground: React.FC<{}> = () => {
  const [playground, setPlayground] = useState<
    typeof import("@apollographql/graphql-playground-react")
  >();

  useEffect(() => {
    import("@apollographql/graphql-playground-react").then(playground => {
      setPlayground(playground);
    });
  }, []);

  if (typeof window === "undefined") {
    return <div>javascript required</div>;
  } else {
    if (!playground) {
      return <div>loading</div>;
    } else {
      return (
        <div>
          <Provider store={playground.store as any}>
            <playground.Playground endpoint={GRAPHQL_ENDPOINT} />
          </Provider>
        </div>
      );
    }
  }
};

export default PlaygroundPage;
