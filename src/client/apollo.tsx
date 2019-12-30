import { ApolloClient } from "apollo-client";
import { HttpLink } from "apollo-link-http";
import { InMemoryCache } from "apollo-cache-inmemory";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

import { GetUserById } from "./__generated__/GetUserById";
import { GetGameBySlug } from "./__generated__/GetGameBySlug";
import { GetRunById } from "./__generated__/GetRunById";

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
          slug
        }
      }
    `,
    { variables: { id } }
  );

export const useGetGameBySlug = (slug: String) =>
  useQuery<GetGameBySlug>(
    gql`
      query GetGameBySlug($slug: String) {
        game(slug: $slug) {
          id
          name
          slug
        }
      }
    `,
    { variables: { slug } }
  );

export const useGetRunById = (id: String) =>
  useQuery<GetRunById>(
    gql`
      query GetRunById($id: String) {
        run(id: $id) {
          id
          timings {
            igt_ms
            rta_ms
            rta_nl_ms
          }
        }
      }
    `,
    { variables: { id } }
  );
