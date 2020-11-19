import React from "react";
import SitePaths from "sitePaths";
import HashAnchor from "sites/Portfolio/components/HashAnchor";

import { StyledProjects } from "./projects.styles";

// TODO: Projects
const Projects = () => (
  <StyledProjects>
    <HashAnchor id={SitePaths.projects} />
    <h1>Projects</h1>
  </StyledProjects>
);

export default Projects;
