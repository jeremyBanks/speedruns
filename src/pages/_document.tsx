import Document, { Head, Html, Main, NextScript } from "next/document";

import Footer from "~/components/footer";
import Header from "~/components/header";
import styles from "~/components/styles.module.scss";

class MyDocument extends Document {
  public render() {
    return (
      <Html className={styles.document}>
        <Head>
          <meta charSet="utf-8" />
          <link rel="icon" href="/icon.png" />
          <link rel="stylesheet" href="/nprogress.css" />
        </Head>
        <body>
          <main>
            <Header />
            <Main />
            <NextScript />
            <Footer />
          </main>
        </body>
      </Html>
    );
  }
}

export default MyDocument;
