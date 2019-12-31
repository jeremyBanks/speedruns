import { ApolloClient } from "apollo-client";
import { HttpLink } from "apollo-link-http";
import { InMemoryCache } from "apollo-cache-inmemory";
import gql from "graphql-tag";
import { useQuery } from "@apollo/react-hooks";

export const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: new HttpLink({
    uri: "http://localhost:3001/"
  })
});

export const useGetMyGames = () => {
  return useQuery(
    gql`
      query GetMyGames($slug: String) {
        war2: game(slug: "war2") {]
          ...MyGameDetails
        }
        war2btdp: game(slug: "war2btdp") {
          ...MyGameDetails
        }
      }

      fragment MyGameDetails on Game {
        id
        name
        slug

        # runs {

        # }
      }
    `
  );
};
