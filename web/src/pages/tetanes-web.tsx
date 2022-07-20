import Layout from "components/layout";
import indexCopy from "data/index.json";
import tetanesCopy from "data/tetanes.json";
import Head from "next/head";

export default function Tetanes() {
  return (
    <>
      <Head>
        <title>{tetanesCopy.title}</title>
        <meta property="og:title" content={tetanesCopy.title} key="title" />
        <meta
          property="og:description"
          content={tetanesCopy.description}
          key="description"
        />
        <meta
          property="og:image"
          content={`${indexCopy.origin}${tetanesCopy.image}`}
          key="image"
        />
      </Head>
      <Layout>
        <section>
          <iframe
            title={tetanesCopy.title}
            src={tetanesCopy.url}
            frameBorder="0"
            width="100%"
            height="1200px"
          ></iframe>
        </section>
      </Layout>
    </>
  );
}
