import { config } from "@fortawesome/fontawesome-svg-core";
import "@fortawesome/fontawesome-svg-core/styles.css";
import ErrorBoundary from "components/errorBoundary";
import copy from "data/index.json";
import "global.css";
import type { AppProps } from "next/app";
import Head from "next/head";

config.autoAddCss = false;

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ErrorBoundary>
      <Head>
        <title>{copy.title}</title>
        <meta name="description" content={copy.description} />
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1.0, minimum-scale=1.0"
        />
      </Head>
      <Component {...pageProps} />
    </ErrorBoundary>
  );
}
