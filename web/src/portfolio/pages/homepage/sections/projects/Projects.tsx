import "./Projects.css";
import HashAnchor from "portfolio/components/HashAnchor";
import projectPosts from "portfolio/data/projectPosts.json";
import { ProjectPost } from "portfolio/models/projectPost";
import { Link } from "react-router-dom";
import routes from "routes.json";

const {
  portfolio: {
    sections: { projects },
  },
} = routes;

type ProjectPostCardProps = {
  post: ProjectPost;
};

const ProjectPostCard = ({ post }: ProjectPostCardProps) => {
  const { publishedOn, url, thumbnail, title } = post;

  return publishedOn ? (
    <article>
      <Link to={url}>
        {thumbnail ? (
          <img
            className="project-thumb"
            src={thumbnail.src}
            alt={thumbnail.alt}
          />
        ) : (
          <p>{title}</p>
        )}
      </Link>
    </article>
  ) : null;
};

const Projects = () => {
  return (
    <section className="projects">
      <HashAnchor id={projects.path.slice(1)} />
      <h2>Projects</h2>
      <section className="project-posts">
        {projectPosts.map((post) => (
          <ProjectPostCard key={post.id} post={post} />
        ))}
      </section>
    </section>
  );
};

export default Projects;
