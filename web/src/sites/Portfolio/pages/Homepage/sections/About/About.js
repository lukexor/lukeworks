import React from "react";
import SitePaths from "sitePaths";
import HashAnchor from "sites/Portfolio/components/HashAnchor";

import { StyledAbout } from "./about.styles";

// TODO: About
const About = () => (
  <StyledAbout>
    <HashAnchor id={SitePaths.about} />
    <h1>About</h1>
  </StyledAbout>
);

export default About;
