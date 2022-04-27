import HashAnchor from "portfolio/components/HashAnchor";
import projectPosts from "portfolio/data/projectPosts.json";
import { Image } from "portfolio/models/post";
import { Link } from "react-router-dom";
import routes from "routes.json";

const {
  portfolio: {
    sections: { projects },
  },
} = routes;

type ProjectPostCardProps = {
  url: string;
  thumbnail: Image;
  title: string;
};

const ProjectPostCard = ({ title, thumbnail, url }: ProjectPostCardProps) => {
  return (
    <article>
      <Link to={url}>
        <div className="card">
          <img className="blur" src={thumbnail.src} alt={thumbnail.alt} />
          <div className="title slide-up">{title}</div>
        </div>
      </Link>
    </article>
  );
};

const Projects = () => {
  return (
    <section className="page-section">
      <HashAnchor id={projects.hash} />
      <h2>Projects</h2>
      <section className="card-grid">
        {[...projectPosts]
          .filter((post) => post.publishedOn && post.thumbnail)
          .slice(0, 9)
          .map(({ id, title, thumbnail, url }) =>
            thumbnail ? (
              <ProjectPostCard
                key={id}
                title={title}
                thumbnail={thumbnail}
                url={url}
              />
            ) : null
          )}
      </section>
    </section>
  );
};

export default Projects;
