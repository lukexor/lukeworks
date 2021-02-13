import React from "react";
import SitePaths from "sitePaths";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import { mockBlogPosts } from "sites/Portfolio/util/mockData";

import {
  BlogPosts,
  BlogThumbnail,
  StyledBlog,
  StyledBlogPostCard,
} from "./blog.styles";

type BlogPost = {
  id: number;
  url: string;
  thumb: string;
  title: string;
  content: string;
  minutesToRead: number;
  publishedAt: string;
  likes: number;
  categoryId: number;
};

type BlogPostCardProps = {
  post: BlogPost;
};

const BlogPostCard: React.FC<BlogPostCardProps> = ({ post }) => {
  return (
    <StyledBlogPostCard>
      <BlogThumbnail src={post.thumb} />
      <h2>{post.title}</h2>
    </StyledBlogPostCard>
  );
};

// TODO: Blog
const Blog: React.FC = () => {
  return (
    <>
      <HashAnchor id={SitePaths.blog} />
      <StyledBlog>
        <h1>Blog</h1>
        <BlogPosts>
          {mockBlogPosts.map((post) => (
            <BlogPostCard key={post.id} post={post} />
          ))}
        </BlogPosts>
      </StyledBlog>
    </>
  );
};

export default Blog;
