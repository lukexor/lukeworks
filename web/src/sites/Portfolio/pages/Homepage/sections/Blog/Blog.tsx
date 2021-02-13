import React from "react";
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
  likes: number;
  publishedOn: string;
  createdOn: string;
  updatedOn: string;
};

type BlogPostCardProps = {
  post: BlogPost;
};

const BlogPostCard: React.FC<BlogPostCardProps> = ({ post }) => {
  return (
    <StyledBlogPostCard>
      {post.thumbnail && <BlogThumbnail src={post.thumbnail} />}
      <h2>{post.title}</h2>
    </StyledBlogPostCard>
  );
};

// TODO: Blog
const Blog: React.FC = () => {
  return (
    <>
      <HashAnchor id={routes.blog.path} />
      <StyledBlog>
        <h1>Blog</h1>
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
