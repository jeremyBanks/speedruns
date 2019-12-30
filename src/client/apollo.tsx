import { ApolloClient } from "apollo-client";
import { HttpLink } from "apollo-link-http";
import { InMemoryCache } from "apollo-cache-inmemory";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

import { TypeNames } from "./__generated__/TypeNames";

export const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: new HttpLink({
    uri: "http://localhost:8080/graphql"
  })
});

export const useTypeNames = () =>
  useQuery<TypeNames>(gql`
    query TypeNames {
      __schema {
        types {
          name
        }
      }
    }
  `);
