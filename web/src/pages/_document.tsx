import { Head, Html, Main, NextScript } from "next/document";

export default function Document() {
  return (
    <Html lang="en">
      <Head>
        <meta charSet="utf-8" />
        <meta name="author" content="Lucas Petherbridge" />
        <link rel="icon" href="/favicon.ico" />
        <meta name="theme-color" content="#6fc3df" />
        <link
          rel="apple-touch-icon"
          sizes="180x180"
          href="/images/icons/apple-touch-icon.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="32x32"
          href="/images/icons/favicon-32x32.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="16x16"
          href="/images/icons/favicon-16x16.png"
        />
        {/* https://developers.google.com/web/fundamentals/web-app-manifest/ */}
        <link rel="manifest" href="/site.webmanifest" />
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link
          rel="preconnect"
          href="https://fonts.gstatic.com"
          crossOrigin="anonymous"
        />
        <link
          href="https://fonts.googleapis.com/css2?family=Rubik&family=Yatra+One&display=swap"
          rel="stylesheet"
        />
      </Head>
      <body>
        <noscript>
          You need to enable JavaScript to render this website correctly.
        </noscript>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
