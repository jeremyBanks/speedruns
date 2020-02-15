import App from "next/app";
import Head from "next/head";
import Router from "next/router";

import styles from "~/components/styles.module.scss";
import { inc, dec } from "~/components/hooks/use-nprogress";
import Header from "~/components/header";
import Footer from "~/components/footer";

Router.events.on("routeChangeStart", inc);
Router.events.on("routeChangeComplete", dec);
Router.events.on("routeChangeError", dec);

// I only need a custom app so I can override the default viewport.
const MyApp = (props: any) => {
  return (
    <>
      <Head>
        <meta name="viewport" content="width=600" />
      </Head>

      <Header />
      <section className={styles.content}>
        <App {...props} />
      </section>
      <Footer />
    </>
  );
};

export default MyApp;
