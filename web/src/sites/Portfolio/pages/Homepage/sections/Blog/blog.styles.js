import styled from "styled-components";

const StyledBlog = styled.section``;

const BlogPosts = styled.section`
  margin: ${(props) => props.theme.sizes.medSmall};
  display: flex;
  justify-content: space-evenly;
  flex-wrap: wrap;
`;

const StyledBlogPostCard = styled.article`
  width: 300px;
  height: 250px;
  background-color: ${(props) => props.theme.colors.backgroundLight};
  background-image: url(${(props) => props.thumb});
  background-position: center;
  background-repeat: no-repeat;
  background-size: cover;
  margin: 15px;
`;

export { BlogPosts, StyledBlog, StyledBlogPostCard };
