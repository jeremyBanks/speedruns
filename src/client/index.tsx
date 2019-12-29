import React from "react";
import { useQuery, ApolloProvider } from "@apollo/react-hooks";
import { GET_TYPE_NAMES, client } from "./apollo";
import { TypeNames } from "./__generated__/TypeNames";

const ClientContent: React.FC = () => {
  const { loading, error, data } = useQuery<TypeNames>(GET_TYPE_NAMES);

  if (!data || loading) {
    return <div>loading...</div>;
  } else if (error) {
    return <div>error: ${JSON.stringify(error)}</div>;
  } else {
    return <div>${JSON.stringify(data)}</div>;
  }
};

export const Client: React.FC = () => (
  <ApolloProvider client={client}>
    <ClientContent />
  </ApolloProvider>
);

export default Client;
