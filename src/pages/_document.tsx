import Document, { Head, Html, Main, NextScript } from "next/document";

import Footer from "../pages-lib/footer";
import Header from "../pages-lib/header";

class MyDocument extends Document {
  public render() {
    return (
      <Html>
        <Head>
          <meta charSet="utf-8" />
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
