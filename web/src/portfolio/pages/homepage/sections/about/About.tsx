import HashAnchor from "portfolio/components/HashAnchor";
import copy from "portfolio/data/copy.json";
import routes from "routes.json";

const {
  menu: { about },
} = routes;

const About = () => (
  <section className="page-section">
    <HashAnchor id={about.hash} />
    <h2>About</h2>
    <p>{copy.About.description}</p>
  </section>
);

export default About;
