import Document, {
  Html,
  Head,
  Main,
  NextScript,
  DocumentContext
} from "next/document";

class MyDocument extends Document {
  static async getInitialProps(context: DocumentContext) {
    const initialProps = await Document.getInitialProps(context);
    return { ...initialProps };
  }

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
            <Main />
            <NextScript />
            <footer>
              <div className="legal">
                All data is from speedrun.com contributors, and is used and
                distributed under the Creative Commons Attribution-NonCommercial
                4.0 International license. See{" "}
                <a href="https://www.speedrun.com/legal">speedrun.com/legal</a>{" "}
                for details.
              </div>
            </footer>
          </main>
        </body>
      </Html>
    );
  }
}

export default MyDocument;
