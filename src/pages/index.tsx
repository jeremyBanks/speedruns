import React from "react";
import { NextPage } from "next";

import HomeContent from "../pages-lib/home";
import { withApollo } from "../pages-lib/with-apollo";

const HomePage: NextPage<{}> = () => <HomeContent />;

export default withApollo(HomePage);
