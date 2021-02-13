import React from "react";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import routes from "sites/Portfolio/routes";
import { StyledContact } from "./contact.styles";

// TODO: Contact
const Contact: React.FC = () => (
  <>
    <HashAnchor id={routes.contact.path} />
    <StyledContact>
      <h1>Contact</h1>
    </StyledContact>
  </>
);

export default Contact;
