import React from "react";
import { Link } from "react-router-dom";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import blogPosts from "sites/Portfolio/data/blogPosts.json";
import { Post } from "sites/Portfolio/models/post";
import routes from "sites/Portfolio/routes.json";
import {
  BlogPosts,
  BlogThumbnail,
  StyledBlog,
  StyledBlogPostCard,
} from "./blog.styles";

type BlogPostCardProps = {
  post: Post;
};

// TODO: Make BlogThumbnail a component with an image placeholder
const BlogPostCard: React.FC<BlogPostCardProps> = ({ post }) => {
  const { url, thumbnail, title } = post;
  return post.publishedOn ? (
    <StyledBlogPostCard>
      <Link to={url}>
        {thumbnail && <BlogThumbnail src={thumbnail.src} alt={thumbnail.alt} />}
        <h3>{title}</h3>
      </Link>
    </StyledBlogPostCard>
  ) : null;
};

const Blog: React.FC = () => {
  return (
    <StyledBlog>
      <HashAnchor id={routes.blog.slice(1)} />
      <h2>Blog</h2>
      <BlogPosts>
        {blogPosts.map((post) => (
          <BlogPostCard key={post.id} post={post} />
        ))}
      </BlogPosts>
    </StyledBlog>
  );
};

export default Blog;
