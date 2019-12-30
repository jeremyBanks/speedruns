import React from "react";
import { ApolloProvider } from "@apollo/react-hooks";
import { client, useTypeNames } from "./apollo";

const ClientContent: React.FC = () => {
  const { loading, error, data } = useTypeNames();

  if (!data || loading) {
    return <pre>loading...</pre>;
  } else if (error) {
    return <pre>error: {JSON.stringify(error, null, 2)}</pre>;
  } else {
    return <pre>{JSON.stringify(data, null, 2)}</pre>;
  }
};

export const Client: React.FC = () => (
  <ApolloProvider client={client}>
    <ClientContent />
  </ApolloProvider>
);

export default Client;