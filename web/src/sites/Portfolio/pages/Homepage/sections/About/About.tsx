import React from "react";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import routes from "sites/Portfolio/routes";
import { StyledAbout } from "./about.styles";

// TODO: About
const About: React.FC = () => (
  <>
    <HashAnchor id={routes.about.path} />
    <StyledAbout>
      <h1>About</h1>
    </StyledAbout>
  </>
);

export default About;
