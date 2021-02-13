import React from "react";
import SitePaths from "sitePaths";
import HashAnchor from "sites/Portfolio/components/HashAnchor";

import { StyledProjects } from "./projects.styles";

// TODO: Projects
const Projects: React.FC = () => (
  <>
    <HashAnchor id={SitePaths.projects} />
    <StyledProjects>
      <h1>Projects</h1>
    </StyledProjects>
  </>
);

export default Projects;
