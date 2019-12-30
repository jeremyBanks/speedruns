import { ApolloClient } from "apollo-client";
import { HttpLink } from "apollo-link-http";
import { InMemoryCache } from "apollo-cache-inmemory";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

import { GetUserById } from "./__generated__/GetUserById";

export const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: new HttpLink({
    uri: "http://localhost:8080/graphql"
  })
});

export const useGetUserById = (id: String) =>
  useQuery<GetUserById>(
    gql`
      query GetUserById($id: String) {
        user(id: $id) {
          id
          name
        }
      }
    `,
    { variables: { id } }
  );
