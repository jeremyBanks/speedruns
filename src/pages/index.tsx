import React from "react";
import { ApolloProvider } from "@apollo/react-hooks";

import { ApolloClient } from "apollo-client";
import { InMemoryCache } from "apollo-cache-inmemory";
import { HttpLink } from "apollo-link-http";
import fetch from "isomorphic-unfetch";

import { MyGamesPage } from "../components/my-games";

export const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: new HttpLink({
    uri: "http://localhost:3001/",
    fetch
  })
});

const Client: React.FC = () => (
  <ApolloProvider client={client}>
    <MyGamesPage />
  </ApolloProvider>
);

export default Client;
