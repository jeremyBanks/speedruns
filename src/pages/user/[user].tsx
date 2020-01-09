import React from "react";
import { useRouter } from "next/router";
import { NextPage } from "next";
import gql from "graphql-tag";

import { withApollo } from "../../pages-lib/with-apollo";
import * as schema from "../../pages-lib/schema";
import useQueryWithStatus from "../../pages-lib/use-query-with-status";

const UserPage: NextPage<{}> = () => {
  const router = useRouter();

  const result = useQueryWithStatus<schema.GetUser>(GetUser, {
    slug: router.query.user
  });

  if (result.data) {
    return <p>{JSON.stringify(result.data)}</p>;
  } else {
    return result.status;
  }
};

export default withApollo(UserPage);

const GetUser = gql`
  query GetUser($slug: String!) {
    user(slug: $slug) {
      id
      slug
    }
  }
`;
