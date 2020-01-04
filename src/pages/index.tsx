import React from "react";
import Head from "next/head";
import { ApolloProvider } from "@apollo/react-hooks";
import { NextPage, NextPageContext } from "next";
import { ApolloClient } from "apollo-client";
import { InMemoryCache, NormalizedCacheObject } from "apollo-cache-inmemory";
import { HttpLink } from "apollo-link-http";
import fetch from "isomorphic-unfetch";
import { getDataFromTree } from "@apollo/react-ssr";

import { MyGamesPage } from "../components/my-games";
import "../components/styles.scss";

const HomePage: NextPage<{
  apolloClient?: ApolloClient<NormalizedCacheObject>;
  apolloCache?: NormalizedCacheObject;
}> = ({ apolloClient, apolloCache }) => {
  if (!apolloClient) {
    apolloClient = new ApolloClient({
      cache: new InMemoryCache().restore(apolloCache || {}),
      link: new HttpLink({
        uri: "http://localhost:3001/",
        fetch
      })
    });
  }

  return (
    <ApolloProvider client={apolloClient}>
      <MyGamesPage />
    </ApolloProvider>
  );
};

export default HomePage;

HomePage.getInitialProps = async (context: NextPageContext) => {
  const { AppTree } = context;

  const onServer = typeof window === "undefined";

  const apolloClient = new ApolloClient({
    ssrMode: onServer,
    cache: new InMemoryCache(),
    link: new HttpLink({
      uri: "http://localhost:3001/",
      fetch
    })
  });

  if (onServer && context.res && context.res.finished) {
    return { apolloClient };
  }

  if (onServer) {
    try {
      await getDataFromTree(<AppTree pageProps={{ apolloClient }} />);
    } catch (error) {
      console.error("Error while running `getDataFromTree`", error);
    }

    // because https://git.io/Jep8E says so:
    Head.rewind();
  }

  const apolloCache = apolloClient.cache.extract();

  return { apolloCache };
};
