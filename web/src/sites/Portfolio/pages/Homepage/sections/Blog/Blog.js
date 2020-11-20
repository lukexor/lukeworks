import PropType from "prop-types";
import React from "react";
import SitePaths from "sitePaths";
import HashAnchor from "sites/Portfolio/components/HashAnchor";

import { BlogPosts, StyledBlog } from "./blog.styles";

const BlogPostCard = ({ post }) => {
  return (
    <div
      style={{
        width: "300px",
        height: "250px",
        border: "1px solid red",
        margin: "15px",
      }}
    >
      <h2>{post.title}</h2>
    </div>
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
  const blogPosts = [
    {
      id: 1,
      url: "/some-test-post",
      thumb: "logo.png",
      title: "Post 1",
      content: "Testing a post",
      minutesToRead: 5,
      publishedAt: "2020-01-02 08:00:00",
      likes: 5,
      categoryId: 1,
    },
    {
      id: 2,
      url: "/some-test-post-2",
      thumb: "logo.png",
      title: "Post 2",
      content: "Testing a post 2",
      minutesToRead: 10,
      publishedAt: "2020-01-05 08:00:00",
      likes: 2,
      categoryId: 2,
    },
    {
      id: 3,
      url: "/some-test-post-3",
      thumb: "logo.png",
      title: "Post 3",
      content: "Testing a post 3",
      minutesToRead: 10,
      publishedAt: "2020-01-05 08:00:00",
      likes: 2,
      categoryId: 2,
    },
    {
      id: 4,
      url: "/some-test-post-4",
      thumb: "logo.png",
      title: "Post 4",
      content: "Testing a post 4",
      minutesToRead: 10,
      publishedAt: "2020-01-05 08:00:00",
      likes: 2,
      categoryId: 2,
    },
  ];

  return (
    <>
      <HashAnchor id={SitePaths.blog} />
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
