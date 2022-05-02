import classnames from "classnames";
import HashAnchor from "portfolio/components/HashAnchor";
import ShowMore from "portfolio/components/post/ShowMore";
import projectPosts from "portfolio/data/projectPosts.json";
import { Image, Post } from "portfolio/models/post";
import { createRef, forwardRef, useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { CSSTransition, TransitionGroup } from "react-transition-group";
import routes from "routes.json";

const {
  menu: { projects },
} = routes;

type ProjectPostCardProps = {
  url: string;
  thumbnail?: Image;
  title: string;
};

const ProjectPostCard = forwardRef<HTMLElement, ProjectPostCardProps>(
  ({ title, thumbnail, url }, ref) => {
    return (
      <article ref={ref}>
        <Link to={url}>
          <div className="card">
            {thumbnail ? (
              <img className="blur" src={thumbnail.src} alt={thumbnail.alt} />
            ) : (
              <div className="card-placeholder" />
            )}
            <div
              className={classnames({ title: true, "slide-up": !!thumbnail })}
            >
              {title}
            </div>
          </div>
        </Link>
      </article>
    );
  }
);

ProjectPostCard.displayName = "ProjectPostCard";

const Projects = () => {
  const [postCount, setPostCount] = useState(6);
  const [totalCount, setTotalCount] = useState(0);
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    const posts = [...projectPosts].filter((post) => post.publishedOn);
    setPosts(posts.slice(0, postCount));
    setTotalCount(posts.length);
  }, [postCount]);

  return (
    <section className="page-section">
      <HashAnchor id={projects.hash} />
      <h2>Projects</h2>
      <section>
        <TransitionGroup component="span" className="card-grid">
          {posts.map(({ id, title, thumbnail, url }) => {
            const postRef = createRef<HTMLElement>();
            return (
              <CSSTransition
                key={id}
                nodeRef={postRef}
                timeout={500}
                classNames="post"
              >
                <ProjectPostCard
                  key={id}
                  ref={postRef}
                  title={title}
                  thumbnail={thumbnail}
                  url={url}
                />
              </CSSTransition>
            );
          })}
        </TransitionGroup>
      </section>
      <ShowMore count={postCount} setCount={setPostCount} total={totalCount} />
    </section>
  );
};

export default Projects;
