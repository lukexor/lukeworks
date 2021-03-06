import React from "react";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import copy from "sites/Portfolio/data/copy.json";
import routes from "sites/Portfolio/routes.json";
import { StyledAbout } from "./about.styles";

const About: React.FC = () => (
  <StyledAbout>
    <HashAnchor id={routes.about.slice(1)} />
    <h2>About</h2>
    <p>{copy.About.description}</p>
  </StyledAbout>
);

export default About;
