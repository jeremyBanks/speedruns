import React from "react";
import ReactDOM from "react-dom";
import { ApolloProvider } from "@apollo/react-hooks";

import { MyGamesPage } from "./my-games";
import { ApolloClient } from "apollo-client";
import { InMemoryCache } from "apollo-cache-inmemory";
import { HttpLink } from "apollo-link-http";

export const client = new ApolloClient({
  cache: new InMemoryCache(),
  link: new HttpLink({
    uri: "http://localhost:3001/",
    fetch: typeof fetch === "function" ? fetch : require("node-fetch")
  })
});

const Client: React.FC = () => (
  <ApolloProvider client={client}>
    <MyGamesPage />
  </ApolloProvider>
);

export default Client;
