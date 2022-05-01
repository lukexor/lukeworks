import HashAnchor from "portfolio/components/HashAnchor";
import copy from "portfolio/data/copy.json";
import routes from "routes.json";

const {
  menu: { contact },
} = routes;
const { content, email } = copy.Contact;

const Contact = () => (
  <section className="page-section">
    <HashAnchor id={contact.hash} />
    <h2>Contact</h2>
    <p>
      {content}{" "}
      <a href={`mailto:${email}`} title={email}>
        {email}
      </a>
      .
    </p>
  </section>
);

export default Contact;
