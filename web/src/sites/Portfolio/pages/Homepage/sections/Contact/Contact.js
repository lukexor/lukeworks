import React from "react";
import SitePaths from "sitePaths";
import HashAnchor from "sites/Portfolio/components/HashAnchor";

import { StyledContact } from "./contact.styles";

// TODO: Contact
const Contact = () => (
  <StyledContact>
    <HashAnchor id={SitePaths.contact} />
    <h1>Contact</h1>
  </StyledContact>
);

export default Contact;
