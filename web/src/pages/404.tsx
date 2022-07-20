import Layout from "components/layout";
import notFoundCopy from "data/404.json";
import contactCopy from "data/contact.json";
import routes from "data/routes.json";
import Head from "next/head";
import Link from "next/link";
import s from "./styles/404.module.css";

export default function NotFound() {
  const { title, message, action1, action2, linkText1, linkText2 } =
    notFoundCopy;
  return (
    <>
      <Head>
        <title>{title}</title>
      </Head>
      <Layout>
        <section className={s.wrapper}>
          <h1>{title}</h1>
          <p>{message}</p>
          <p>
            {action1}
            <a href={`mailto:${contactCopy.email}`} title={contactCopy.email}>
              {linkText1}
            </a>
            {action2}
            <Link href={routes.home.path} title={routes.home.title}>
              {linkText2}
            </Link>
            .
          </p>
        </section>
      </Layout>
    </>
  );
}
