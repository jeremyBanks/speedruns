import React from "react";
import { useRouter } from "next/router";
import { NextPage } from "next";

const RunPage: NextPage = () => {
  const router = useRouter();

  return (
    <p>
      game: {router.query.game}, run: {router.query.run}
    </p>
  );
};

export default RunPage;
