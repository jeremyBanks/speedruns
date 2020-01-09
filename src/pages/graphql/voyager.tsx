import { useState, useEffect } from "react";
import { NextPage } from "next";

import styles from "../../pages-lib/styles.module.scss";
import introspection from "../../../public/graphql/schema.json";

const VoyagerPage: NextPage<{ introspectionData: object }> = () => {
  const [Voyager, setVoyager] = useState<
    typeof import("graphql-voyager").Voyager
  >();

  useEffect(() => {
    import("graphql-voyager").then(({ Voyager }) => setVoyager(() => Voyager));
  }, []);

  if (typeof window === "undefined") {
    return <div>javascript required</div>;
  } else {
    if (!Voyager) {
      return <div>loading</div>;
    } else {
      return (
        <div className={styles.voyagerFrame}>
          <link rel="stylesheet" href="/graphql/voyager.css" />
          <Voyager
            introspection={async () => ({
              data: introspection
            })}
            workerURI="/graphql/voyager.worker.js"
            displayOptions={{
              skipRelay: false
            }}
            hideSettings={true}
          />
        </div>
      );
    }
  }
};

export default VoyagerPage;
