import React from "react";
import { useRouter } from "next/router";
import { NextPage } from "next";
import gql from "graphql-tag";

import { withApollo } from "../../../pages-lib/with-apollo";
import * as schema from "../../../pages-lib/schema";
import useQueryWithStatus from "../../../pages-lib/use-query-with-status";

const RunPage: NextPage<{}> = () => {
  const router = useRouter();

  const result = useQueryWithStatus<schema.GetRun>(GetRun, {
    id: router.query.run
  });

  if (result.data) {
    return <p>{JSON.stringify(result.data)}</p>;
  } else {
    return result.status;
  }
};

export default withApollo(RunPage);

const GetRun = gql`
  query GetRun($id: String!) {
    run(id: $id) {
      id
    }
  }
`;
