import "./About.css";
import HashAnchor from "portfolio/components/HashAnchor";
import copy from "portfolio/data/copy.json";
import routes from "routes.json";

const {
  portfolio: {
    sections: { about },
  },
} = routes;

const About = () => (
  <section className="about">
    <HashAnchor id={about.path.slice(1)} />
    <h2>About</h2>
    <p>{copy.About.description}</p>
  </section>
);

export default About;
