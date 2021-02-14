import React from "react";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import copy from "sites/Portfolio/data/copy.json";
import routes from "sites/Portfolio/routes";
import { StyledAbout } from "./about.styles";

// TODO: About
const About: React.FC = () => (
  <>
    <HashAnchor id={routes.about.path} />
    <StyledAbout>
      <h2>About</h2>
      <p>{copy.About.description}</p>
    </StyledAbout>
  </>
);

export default About;
