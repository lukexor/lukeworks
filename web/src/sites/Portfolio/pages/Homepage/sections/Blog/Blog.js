import React from "react";
import SitePaths from "sitePaths";
import HashAnchor from "sites/Portfolio/components/HashAnchor";

import { StyledBlog } from "./blog.styles";

// TODO: Blog
const Blog = () => (
  <>
    <HashAnchor id={SitePaths.blog} />
    <StyledBlog>
      <h1>Blog</h1>
    </StyledBlog>
  </>
);

export default Blog;
