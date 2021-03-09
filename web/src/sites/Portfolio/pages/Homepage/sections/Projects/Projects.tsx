import React from "react";
import { Link } from "react-router-dom";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import projectPosts from "sites/Portfolio/data/projectPosts.json";
import { ProjectPost } from "sites/Portfolio/models/projectPost";
import routes from "sites/Portfolio/routes.json";
import {
  ProjectPosts,
  ProjectThumbnail,
  StyledProjectPostCard,
  StyledProjects,
} from "./projects.styles";

type ProjectPostCardProps = {
  post: ProjectPost;
};

const ProjectPostCard: React.FC<ProjectPostCardProps> = ({ post }) => {
  const { publishedOn, url, thumbnail, title } = post;

  return publishedOn ? (
    <StyledProjectPostCard>
      <Link to={url}>
        {thumbnail.src ? (
          <ProjectThumbnail src={thumbnail.src} alt={thumbnail.alt} />
        ) : (
          <p>{title}</p>
        )}
      </Link>
    </StyledProjectPostCard>
  ) : null;
};

const Projects: React.FC = () => (
  <StyledProjects>
    <HashAnchor id={routes.projects.slice(1)} />
    <h2>Projects</h2>
    <ProjectPosts>
      {projectPosts.map((post) => (
        <ProjectPostCard key={post.id} post={post} />
      ))}
    </ProjectPosts>
  </StyledProjects>
);

export default Projects;
