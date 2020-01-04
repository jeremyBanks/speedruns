import { withSPARouter } from "next-spa/router";

import "../pages-lib/global.scss";

// XXX: using `any` because I'm just importing some styles and don't
// want to look it up now. fix this if you do anything else here.
const MyApp = ({ Component, pageProps }: any) => {
  return <Component {...pageProps} />;
};

export default withSPARouter(MyApp);
