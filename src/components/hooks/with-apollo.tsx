import { ApolloProvider } from "@apollo/react-hooks";
import { getDataFromTree } from "@apollo/react-ssr";
import {
  InMemoryCache,
  NormalizedCacheObject,
  defaultDataIdFromObject,
} from "apollo-cache-inmemory";
import ApolloClient from "apollo-client";
import { HttpLink } from "apollo-link-http";
import fetch from "isomorphic-unfetch";
import { NextPage, NextPageContext } from "next";
import Head from "next/head";
import React from "react";

const onNode = typeof window === "undefined";
const onNodeDev = onNode && process.env.NODE_ENV === "development";
const inBrowser = !onNode;
const inBrowserDev = inBrowser && window.location.protocol !== "https:";
export const DEBUG = inBrowserDev || onNodeDev;

export const GRAPHQL_ENDPOINT = DEBUG
  ? "http://localhost:3001/graphql"
  : "https://graphql-v0.speedrun.ca/graphql";

let globalApolloClient: ApolloClient<NormalizedCacheObject> | undefined;

const getApolloClient = (
  initialState?: NormalizedCacheObject,
): ApolloClient<NormalizedCacheObject> => {
  if (onNode || !globalApolloClient) {
    const cache = new InMemoryCache({
      dataIdFromObject: (o: any) => {
        if (o && o.id && o.srcId) {
          return o.id;
        } else {
          return defaultDataIdFromObject(o);
        }
      },
    });

    if (initialState) {
      cache.restore(initialState);
    }

    const policy = "cache-and-network";

    globalApolloClient = new ApolloClient({
      cache,
      defaultOptions: {
        query: {
          fetchPolicy: policy,
        },
        watchQuery: {
          fetchPolicy: policy,
        },
      } as any,
      link: new HttpLink({ uri: GRAPHQL_ENDPOINT, fetch }),
      ssrMode: onNode,
    });
  }

  return globalApolloClient;
};

type ApolloNextPage = NextPage<{
  apolloClient?: ApolloClient<NormalizedCacheObject>;
  apolloCache?: NormalizedCacheObject;
}>;

export const withApollo = (Page: NextPage<{}>): ApolloNextPage => {
  const WithApollo: ApolloNextPage = ({ apolloCache, apolloClient }) => {
    if (!apolloClient) {
      apolloClient = getApolloClient(apolloCache);
    }
    return (
      <ApolloProvider client={apolloClient}>
        <Page />
      </ApolloProvider>
    );
  };

  WithApollo.getInitialProps = async (context: NextPageContext) => {
    const { AppTree } = context;

    const apolloClient = getApolloClient();

    const props: {
      apolloCache?: NormalizedCacheObject;
    } = {};

    if (onNode) {
      try {
        await getDataFromTree(<AppTree pageProps={{ apolloClient }} />);
      } catch (error) {
        console.error("Error while running `getDataFromTree`", error);
      }

      // because https://git.io/Jep8E says so:
      Head.rewind();

      props.apolloCache = apolloClient.cache.extract();
    }

    return props;
  };

  return WithApollo;
};
