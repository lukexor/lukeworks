import React from "react";
import SitePaths from "sitePaths";
import HashAnchor from "sites/Portfolio/components/HashAnchor";

import { StyledContact } from "./contact.styles";

// TODO: Contact
const Contact: React.FC = () => (
  <>
    <HashAnchor id={SitePaths.contact} />
    <StyledContact>
      <h1>Contact</h1>
    </StyledContact>
  </>
);

export default Contact;
