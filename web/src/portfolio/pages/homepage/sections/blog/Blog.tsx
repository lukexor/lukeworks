import HashAnchor from "portfolio/components/HashAnchor";
import ShowMore from "portfolio/components/post/ShowMore";
import blogPosts from "portfolio/data/blogPosts.json";
import { Image, Post } from "portfolio/models/post";
import { createRef, useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { CSSTransition, TransitionGroup } from "react-transition-group";
import routes from "routes.json";

const {
  menu: { blog },
} = routes;

type BlogPostCardProps = {
  url: string;
  thumbnail: Image;
  title: string;
};

const BlogPostCard = ({ title, thumbnail, url }: BlogPostCardProps) => {
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

const Blog = () => {
  const [postCount, setPostCount] = useState(9);
  const [totalCount, setTotalCount] = useState(0);
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    const posts = [...blogPosts].filter(
      (post) => post.publishedOn && post.thumbnail
    );
    setPosts(posts.slice(0, postCount));
    setTotalCount(posts.length);
  }, [postCount]);

  return (
    <section className="page-section">
      <HashAnchor id={blog.hash} />
      <h2>Blog</h2>
      <section>
        <TransitionGroup component="span" className="card-grid">
          {posts.map(({ id, title, thumbnail, url }) => {
            const postRef = createRef<HTMLElement>();
            return thumbnail ? (
              <CSSTransition
                key={id}
                nodeRef={postRef}
                timeout={500}
                classNames="post"
              >
                <BlogPostCard
                  key={id}
                  title={title}
                  thumbnail={thumbnail}
                  url={url}
                />
              </CSSTransition>
            ) : (
              <></>
            );
          })}
        </TransitionGroup>
      </section>
      <ShowMore count={postCount} setCount={setPostCount} total={totalCount} />
    </section>
  );
};

export default Blog;
