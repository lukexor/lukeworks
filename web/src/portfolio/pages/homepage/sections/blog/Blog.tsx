import "./Blog.css";
import HashAnchor from "portfolio/components/HashAnchor";
import blogPosts from "portfolio/data/blogPosts.json";
import { Post } from "portfolio/models/post";
import { Link } from "react-router-dom";
import routes from "routes.json";

const {
  portfolio: {
    sections: { blog },
  },
} = routes;

type BlogPostCardProps = {
  post: Post;
};

// TODO: Make BlogThumbnail a component with an image placeholder
const BlogPostCard = ({ post }: BlogPostCardProps) => {
  const { url, thumbnail, title } = post;
  return post.publishedOn ? (
    <article>
      <Link to={url}>
        {thumbnail ? (
          <img className="blog-thumb" src={thumbnail.src} alt={thumbnail.alt} />
        ) : (
          <p>{title}</p>
        )}
      </Link>
    </article>
  ) : null;
};

const Blog = () => {
  return (
    <section className="blog">
      <HashAnchor id={blog.path.slice(1)} />
      <h2>Blog</h2>
      <section className="blog-posts">
        {blogPosts.map((post) => (
          <BlogPostCard key={post.id} post={post} />
        ))}
      </section>
    </section>
  );
};

export default Blog;
