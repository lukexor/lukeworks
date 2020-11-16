import React from "react";
import SitePaths from "sitePaths";

import { StyledBlog } from "./blog.styles";

const Blog = () => (
  <StyledBlog>
    <a id={SitePaths.blog.replace("#", "")} className="anchor"></a>
    <h1>Blog</h1>
  </StyledBlog>
);

export default Blog;
