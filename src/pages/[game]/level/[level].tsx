import { NextPage } from "next";
import { useRouter } from "next/router";
import React from "react";

const LevelPage: NextPage = () => {
  const router = useRouter();

  return (
    <p>
      game: {router.query.game}, level: {router.query.level}
    </p>
  );
};

export default LevelPage;
