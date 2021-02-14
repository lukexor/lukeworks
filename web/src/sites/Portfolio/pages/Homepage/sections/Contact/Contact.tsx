import React from "react";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import copy from "sites/Portfolio/data/copy.json";
import routes from "sites/Portfolio/routes";
import { StyledContact } from "./contact.styles";

// TODO: Contact
const Contact: React.FC = () => (
  <>
    <HashAnchor id={routes.contact.path} />
    <StyledContact>
      <h2>Contact</h2>
      <p>
        {copy.Contact.content}{" "}
        <a href={`mailto:${copy.Contact.email}`}>{copy.Contact.email}</a>.
      </p>
    </StyledContact>
  </>
);

export default Contact;
