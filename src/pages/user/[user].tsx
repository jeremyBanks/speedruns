import React from "react";
import { useRouter } from "next/router";
import { NextPage } from "next";

const UserPage: NextPage = () => {
  const router = useRouter();

  return <p>user: {router.query.user}</p>;
};

export default UserPage;
