import Document, { Html, Head, Main, NextScript } from "next/document";

import styles from "./styles.module.scss";
import Header from "../pages-lib/header";
import Footer from "../pages-lib/footer";

class MyDocument extends Document {
  render() {
    return (
      <Html>
        <Head>
          <meta charSet="utf-8" />
          <meta
            httpEquiv="Content-Security-Policy"
            defaultValue="default-src 'self'; img-src *"
          />
          <meta name="viewport" content="width=682, initial-scale=0" />
          <link rel="icon" href="/contest.png" />
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
