import Layout from "components/layout";
import errorCopy from "data/500.json";
import contactCopy from "data/contact.json";
import routes from "data/routes.json";
import Link from "next/link";
import s from "./styles/500.module.css";

export default function ServerError() {
  const { title, message, action1, action2, linkText1, linkText2 } = errorCopy;
  return (
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
  );
}
