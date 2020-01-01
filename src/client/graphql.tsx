import { ApolloClient } from "apollo-client";
import { HttpLink } from "apollo-link-http";
import { InMemoryCache } from "apollo-cache-inmemory";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

import * as types from "./graphql-types";

export const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: new HttpLink({
    uri: "http://localhost:3001/"
  })
});

export const useGetMyGames = () => {
  return useQuery<types.GetMyGames>(GetMyGames);
};

const MyGameDetails = gql`
  fragment MyGameDetails on Game {
    id
    name
    slug
    runs {
      id
    }
  }
`;

const GetMyGames = gql`
  query GetMyGames {
    war2: game(slug: "wc2") {
      ...MyGameDetails
    }
    war2btdp: game(slug: "wc2btdp") {
      ...MyGameDetails
    }
  }

  ${MyGameDetails}
`;
