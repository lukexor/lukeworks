import styled from "styled-components";

const StyledBlog = styled.section`
  padding-top: ${(props) => props.theme.sizes.xlarge};
`;

const BlogPosts = styled.div`
  margin: ${(props) => props.theme.sizes.medSmall};
  display: flex;
  justify-content: space-evenly;
  flex-wrap: wrap;
`;

export { BlogPosts, StyledBlog };
