import React from "react";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import routes from "sites/Portfolio/routes";
import { StyledProjects } from "./projects.styles";

// TODO: Projects
const Projects: React.FC = () => (
  <>
    <HashAnchor id={routes.projects.path} />
    <StyledProjects>
      <h1>Projects</h1>
    </StyledProjects>
  </>
);

export default Projects;
