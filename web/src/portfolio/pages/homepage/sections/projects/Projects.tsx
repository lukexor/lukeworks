import classnames from "classnames";
import HashAnchor from "portfolio/components/HashAnchor";
import projectPosts from "portfolio/data/projectPosts.json";
import { Image } from "portfolio/models/post";
import { useState } from "react";
import { Link } from "react-router-dom";
import routes from "routes.json";

const {
  menu: { projects },
} = routes;

type ProjectPostCardProps = {
  url: string;
  thumbnail?: Image;
  title: string;
};

const ProjectPostCard = ({ title, thumbnail, url }: ProjectPostCardProps) => {
  return (
    <article>
      <Link to={url}>
        <div className="card">
          {thumbnail ? (
            <img className="blur" src={thumbnail.src} alt={thumbnail.alt} />
          ) : (
            <div className="card-placeholder" />
          )}
          <div className={classnames({ title: true, "slide-up": !!thumbnail })}>
            {title}
          </div>
        </div>
      </Link>
    </article>
  );
};

const Projects = () => {
  const [posts] = useState(() =>
    [...projectPosts].filter((post) => post.publishedOn).slice(0, 6)
  );
  return (
    <section className="page-section">
      <HashAnchor id={projects.hash} />
      <h2>Projects</h2>
      <section className="card-grid">
        {posts.map(({ id, title, thumbnail, url }) => (
          <ProjectPostCard
            key={id}
            title={title}
            thumbnail={thumbnail}
            url={url}
          />
        ))}
      </section>
    </section>
  );
};

export default Projects;
