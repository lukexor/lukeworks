import HashAnchor from "components/hashAnchor";
import copy from "data/about.json";
import routes from "data/routes.json";
import s from "./homepage.module.css";

export default function About() {
  return (
    <section className={s.pageSection}>
      <HashAnchor id={routes.menu.about.hash} />
      <h2>About</h2>
      <p>{copy.description}</p>
    </section>
  );
}
