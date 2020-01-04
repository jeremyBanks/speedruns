import React from "react";
import { useRouter } from "next/router";
import { NextPage } from "next";

const LevelPage: NextPage = () => {
  const router = useRouter();

  return (
    <p>
      game: {router.query.game}, level: {router.query.level}
    </p>
  );
};

export default LevelPage;
