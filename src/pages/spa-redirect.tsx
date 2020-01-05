import React from "react";
import { useRouter } from "next/router";
import { NextPage } from "next";
import Link from "next/link";

const SpaRedirectPage: NextPage<{}> = () => {
  const router = useRouter();

  let path;
  if (typeof window === "undefined") {
    path = "/";
    router.push("/");
  } else {
    path = window.location.pathname;
    router.replace(path);
  }
  return (
    <Link href={path}>
      <a>redirect</a>
    </Link>
  );
};

export default SpaRedirectPage;
