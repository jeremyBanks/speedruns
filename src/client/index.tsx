import React from "react";
import ReactDOM from "react-dom";
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
    let me = data.banks;
    let games: Array<MyGameDetails> = [data.war2, data.war2btdp];

    return (
      <>
        <h1>
          {me.id} {me.slug}
        </h1>

        {games.map(game => (
          <>
            <h1>
              {game.id} {game.name}
            </h1>

            <h2>Leaderboard</h2>

            <ol>
              {game.leaderboard.map(rank => (
                <li value={rank.tiedRank}>
                  {rank.timeMs} {rank.run.id}
                </li>
              ))}
            </ol>

            <h2>All Runs</h2>

            <ul>
              {game.runs.map(run => (
                <li>
                  Run {run.id} in {run.category.id} {run.category.slug}{" "}
                  {run.category.name}{" "}
                  {run.level && (
                    <>
                      {run.level.id} {run.level.slug} {run.level.name}
                    </>
                  )}
                </li>
              ))}
            </ul>
          </>
        ))}
      </>
    );
  }
};

const Client: React.FC = () => (
  <ApolloProvider client={client}>
    <ClientContent />
  </ApolloProvider>
);

ReactDOM.render(<Client />, document.querySelector("main"));
