import Layout from "components/layout";
import indexCopy from "data/index.json";
import tetanesCopy from "data/tetanes.json";
import Head from "next/head";
import { useEffect, useRef } from "react";

export default function Tetanes() {
  const ref = useRef<HTMLIFrameElement>(null);

  // Super hacky way to keep the iframe height in sync with its content
  useEffect(() => {
    let timer = setInterval(() => {
      const iframe = ref.current;
      if (iframe && iframe.contentWindow) {
        const newHeight =
          iframe.contentWindow.document.body.scrollHeight + 200 + "px";
        if (iframe.height !== newHeight) {
          iframe.height = newHeight;
        }
      }
    }, 1000);

    return () => {
      clearInterval(timer);
    };
  });

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
            ref={ref}
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
