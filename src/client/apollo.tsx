import { ApolloClient } from "apollo-client";
import { HttpLink } from "apollo-link-http";
import { InMemoryCache } from "apollo-cache-inmemory";
import gql from "graphql-tag";

const cache = new InMemoryCache();
const link = new HttpLink({
  uri: "http://localhost:8080/graphql"
});

export const client = new ApolloClient({
  cache,
  link
});

export const GET_TYPE_NAMES = gql`
  query TypeNames {
    __schema {
      types {
        name
      }
    }
  }
`;
