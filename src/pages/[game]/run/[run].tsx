import React from "react";
import { useRouter } from "next/router";
import { NextPage } from "next";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

import { withApollo } from "../../../pages-lib/with-apollo";
import * as schema from "../../../pages-lib/schema";

const RunPage: NextPage<{}> = () => {
  const router = useRouter();
  const { loading, error, data } = useQuery<schema.GetRun>(GetRun, {
    variables: { id: router.query.run }
  });

  if (data && !error) {
    return <p>{JSON.stringify(data)}</p>;
  } else {
    return <p>loading: {String(loading || error)}</p>;
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
