import { NextPage } from "next";
import { useRouter } from "next/router";
import React from "react";
import { withApollo } from "../../pages-lib/with-apollo";

const GamePage: NextPage = () => {
  const router = useRouter();

  return <p>game: {router.query.game}</p>;
};

export default withApollo(GamePage);
