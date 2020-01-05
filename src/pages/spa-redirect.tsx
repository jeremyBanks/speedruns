import React from "react";
import { useRouter } from "next/router";
import { NextPage } from "next";
import Link from "next/link";

const SpaRedirectPage: NextPage<{}> = () => {
  const router = useRouter();

  let path;
  if (typeof window === "undefined") {
    path = "/";
  } else {
    path = window.location.pathname;
    if (path === "/spa-redirect") {
      path = "/";
    }
    router.replace(path);
  }

  return (
    <Link href={path}>
      <a>redirect</a>
    </Link>
  );
};

export default SpaRedirectPage;
