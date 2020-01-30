import App from "next/app";
import Head from "next/head";

// I only need a custom app so I can override the default viewport.
const MyApp = (props: any) => {
  return (
    <>
      <Head>
        <meta name="viewport" content="width=640" />
      </Head>
      <App {...props} />
    </>
  );
};

export default MyApp;
