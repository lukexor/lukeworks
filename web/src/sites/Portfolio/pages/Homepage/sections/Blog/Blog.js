import PropType from "prop-types";
import React from "react";
import SitePaths from "sitePaths";
import HashAnchor from "sites/Portfolio/components/HashAnchor";
import { mockBlogPosts } from "sites/Portfolio/util/mockData";

import { BlogPosts, StyledBlog, StyledBlogPostCard } from "./blog.styles";

const BlogPostCard = ({ post }) => {
  return (
    <StyledBlogPostCard thumb={post.thumb}>
      <h2>{post.title}</h2>
    </StyledBlogPostCard>
  );
};

BlogPostCard.propTypes = {
  post: PropType.exact({
    id: PropType.number.isRequired,
    url: PropType.string.isRequired,
    thumb: PropType.string.isRequired,
    title: PropType.string.isRequired,
    content: PropType.string.isRequired,
    minutesToRead: PropType.number.isRequired,
    publishedAt: PropType.string.isRequired,
    likes: PropType.number.isRequired,
    categoryId: PropType.number.isRequired,
  }),
};

// TODO: Blog
const Blog = () => {
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
