import HashAnchor from "portfolio/components/HashAnchor";
import blogPosts from "portfolio/data/blogPosts.json";
import { Image } from "portfolio/models/post";
import { useState } from "react";
import { Link } from "react-router-dom";
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
  const [posts] = useState(() =>
    [...blogPosts]
      .filter((post) => post.publishedOn && post.thumbnail)
      .slice(0, 9)
  );
  return (
    <section className="page-section">
      <HashAnchor id={blog.hash} />
      <h2>Blog</h2>
      <section className="card-grid">
        {posts.map(({ id, title, thumbnail, url }) =>
          thumbnail ? (
            <BlogPostCard
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

export default Blog;
