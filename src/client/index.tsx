import React from "react";
import { ApolloProvider } from "@apollo/react-hooks";

import { client, useGetMyGames } from "./graphql";
import { MyGameDetails } from "./graphql-types";

const ClientContent: React.FC = () => {
  const { loading, error, data } = useGetMyGames();

  if (!data || loading) {
    return <pre>loading...</pre>;
  } else if (error) {
    return <pre>error: {JSON.stringify(error, null, 2)}</pre>;
  } else {
    let games: Array<MyGameDetails> = [data.war2, data.war2btdp];

    return (
      <>
        {games.map(game => (
          <>
            <h2>{game.name}</h2>

            <ul>
              {game.runs.map(run => (
                <li>Run {run.id}</li>
              ))}
            </ul>
          </>
        ))}
      </>
    );
  }
};

export const Client: React.FC = () => (
  <ApolloProvider client={client}>
    <ClientContent />
  </ApolloProvider>
);

export default Client;
