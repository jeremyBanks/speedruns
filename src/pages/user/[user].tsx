import React from "react";
import { useRouter } from "next/router";
import { NextPage } from "next";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

import { withApollo } from "../../pages-lib/with-apollo";
import * as schema from "../../pages-lib/schema";

const UserPage: NextPage<{}> = () => {
  const router = useRouter();
  const { loading, error, data } = useQuery<schema.GetUser>(GetUser, {
    variables: { slug: router.query.user }
  });

  if (data && !error) {
    return <p>{JSON.stringify(data)}</p>;
  } else {
    return <p>loading: {String(loading || error)}</p>;
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
