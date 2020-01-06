import fetch from "isomorphic-unfetch";
import { NextPage } from "next";
import { useState, useEffect } from "react";

const VoyagerPage: NextPage = () => {
  const [Voyager, setVoyager] = useState<
    typeof import("graphql-voyager").Voyager
  >();

  useEffect(() => {
    import("graphql-voyager").then(({ Voyager }) => setVoyager(Voyager));
  }, []);

  if (typeof window === "undefined") {
    return <div>javascript required</div>;
  } else {
    if (!Voyager) {
      return <div>loading</div>;
    } else {
      return (
        <Voyager
          introspection={(query: unknown) =>
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

export default VoyagerPage;
