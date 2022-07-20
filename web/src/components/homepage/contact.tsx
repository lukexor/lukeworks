import HashAnchor from "components/hashAnchor";
import copy from "data/contact.json";
import routes from "data/routes.json";
import s from "./homepage.module.css";

export default function Contact() {
  return (
    <section className={s.pageSection}>
      <HashAnchor id={routes.menu.contact.hash} />
      <h2>Contact</h2>
      <p>
        {copy.content}{" "}
        <a href={`mailto:${copy.email}`} title={copy.email}>
          {copy.email}
        </a>
        .
      </p>
    </section>
  );
}
