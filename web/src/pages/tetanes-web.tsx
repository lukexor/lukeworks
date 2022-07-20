import Layout from "components/layout";
import copy from "data/tetanes.json";
import Head from "next/head";

export default function Tetanes() {
  return (
    <>
      <Head>
        <title>{copy.title}</title>
        <meta property="og:title" content={copy.title} key="title" />
        <meta
          property="og:description"
          content={copy.description}
          key="description"
        />
        <meta property="og:image" content={copy.image} key="image" />
      </Head>
      <Layout>
        <section>
          <iframe
            title={copy.title}
            src={copy.url}
            frameBorder="0"
            width="100%"
            height="1200px"
          ></iframe>
        </section>
      </Layout>
    </>
  );
}
