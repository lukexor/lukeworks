import React from "react";
import { Link } from "react-router-dom";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import projectPosts from "sites/Portfolio/data/projectPosts.json";
import routes from "sites/Portfolio/routes.json";
import {
  ProjectPosts,
  ProjectThumbnail,
  StyledProjectPostCard,
  StyledProjects,
} from "./projects.styles";

type ProjectPost = {
  id: number;
  url: string;
  title: string;
  thumbnail: Maybe<string>;
  image: Maybe<string>;
  imageAlt: Maybe<string>;
  content: string;
  website: Maybe<string>;
  tags: string[];
  likes: number;
  startedOn: Maybe<string>;
  completedOn: Maybe<string>;
  publishedOn: Maybe<string>;
  createdOn: string;
  updatedOn: string;
};

type ProjectPostCardProps = {
  post: ProjectPost;
};

const ProjectPostCard: React.FC<ProjectPostCardProps> = ({ post }) => {
  return post.publishedOn ? (
    <StyledProjectPostCard>
      <Link to={post.url}>
        <ProjectThumbnail title={post.title} src={post.thumbnail} />
      </Link>
    </StyledProjectPostCard>
  ) : null;
};

// TODO: Projects
const Projects: React.FC = () => (
  <>
    <StyledProjects>
      <HashAnchor id={routes.Projects} />
      <h2>Projects</h2>
      <ProjectPosts>
        {projectPosts.map((post) => (
          <ProjectPostCard key={post.id} post={post} />
        ))}
      </ProjectPosts>
    </StyledProjects>
  </>
);

export default Projects;
