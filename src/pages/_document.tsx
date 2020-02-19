import Document, { Head, Html, Main, NextScript } from "next/document";

import styles from "~/components/styles.module.scss";

class MyDocument extends Document {
  public render() {
    return (
      <Html className={styles.document}>
        <Head>
          <meta charSet="utf-8" />
          <link rel="icon" href="/illufinch.png" />
        </Head>
        <body>
          <main>
            <Main />
            <NextScript />
          </main>
        </body>
      </Html>
    );
  }
}

export default MyDocument;
