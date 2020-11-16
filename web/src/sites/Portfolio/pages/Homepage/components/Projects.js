import React from "react";
import SitePaths from "sitePaths";

import { StyledProjects } from "./projects.styles";

const Projects = () => (
  <StyledProjects>
    <a id={SitePaths.projects.replace("#", "")} className="anchor"></a>
    <h1>Projects</h1>
  </StyledProjects>
);

export default Projects;
