import * as React from "react";
import { Provider } from "react-redux";
import { NextPage } from "next";
import { useState, useEffect } from "react";

const PlaygroundPage: NextPage = () => {
  const [playground, setPlayground] = useState<
    typeof import("@apollographql/graphql-playground-react")
  >();

  useEffect(() => {
    import("@apollographql/graphql-playground-react").then(playground =>
      setPlayground(playground)
    );
  }, []);

  if (typeof window === "undefined") {
    return <div>javascript required</div>;
  } else {
    if (!playground) {
      return <div>loading</div>;
    } else {
      return (
        <Provider store={playground.store as any}>
          <playground.Playground endpoint="http://localhost:3001/" />
        </Provider>
      );
    }
  }
};

export default PlaygroundPage;
