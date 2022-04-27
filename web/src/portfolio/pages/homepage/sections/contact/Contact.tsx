import HashAnchor from "portfolio/components/HashAnchor";
import copy from "portfolio/data/copy.json";
import routes from "routes.json";

const {
  portfolio: {
    sections: { contact },
  },
} = routes;

const Contact = () => (
  <section className="page-section">
    <HashAnchor id={contact.hash} />
    <h2>Contact</h2>
    <p>
      {copy.Contact.content}{" "}
      <a href={`mailto:${copy.Contact.email}`}>{copy.Contact.email}</a>.
    </p>
  </section>
);

export default Contact;
