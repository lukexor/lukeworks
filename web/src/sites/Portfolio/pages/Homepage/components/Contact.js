import React from "react";
import SitePaths from "sitePaths";

import { StyledContact } from "./contact.styles";

const Contact = () => (
  <StyledContact>
    <a id={SitePaths.contact.replace("#", "")} className="anchor"></a>
    <h1>Contact</h1>
  </StyledContact>
);

export default Contact;
