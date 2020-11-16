import React from "react";
import SitePaths from "sitePaths";

import { StyledAbout } from "./about.styles";

const About = () => (
  <StyledAbout>
    <a id={SitePaths.about.replace("#", "")} className="anchor"></a>
    <h1>About</h1>
  </StyledAbout>
);

export default About;
