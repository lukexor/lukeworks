import React from "react";
import { Link } from "react-router-dom";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import blogPosts from "sites/Portfolio/data/blogPosts.json";
import routes from "sites/Portfolio/routes";
import {
  BlogPosts,
  BlogThumbnail,
  StyledBlog,
  StyledBlogPostCard,
} from "./blog.styles";

type BlogPost = {
  id: number;
  url: string;
  title: string;
  thumbnail: Maybe<string>;
  image: Maybe<string>;
  imageAlt: Maybe<string>;
  content: string;
  minutesToRead: number;
  category: string;
  tags: string[];
  likes: number;
  publishedOn: Maybe<string>;
  createdOn: string;
  updatedOn: string;
};

type BlogPostCardProps = {
  post: BlogPost;
};

const BlogPostCard: React.FC<BlogPostCardProps> = ({ post }) => {
  return post.publishedOn ? (
    <StyledBlogPostCard>
      <Link to={post.url}>
        <BlogThumbnail src={post.thumbnail} />
        <h3>{post.title}</h3>
      </Link>
    </StyledBlogPostCard>
  ) : null;
};

// TODO: Blog
const Blog: React.FC = () => {
  return (
    <>
      <HashAnchor id={routes.blog.path} />
      <StyledBlog>
        <h2>Blog</h2>
        <BlogPosts>
          {blogPosts.map((post) => (
            <BlogPostCard key={post.id} post={post} />
          ))}
        </BlogPosts>
      </StyledBlog>
    </>
  );
};

export default Blog;
