import styled from "styled-components";

const thumbWidth = "300px";

const StyledBlog = styled.section``;

const BlogPosts = styled.section`
  margin: ${(props) => props.theme.sizes.medSmall};
  display: flex;
  justify-content: space-evenly;
  flex-wrap: wrap;
`;

const StyledBlogPostCard = styled.article`
  max-width: ${thumbWidth};
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 15px;

  h2 {
    font-size: ${(props) => props.theme.sizes.large};
  }
`;

const BlogThumbnail = styled.div<{ src: string; }>`
  width: ${thumbWidth};
  height: 250px;
  background-color: ${(props) => props.theme.colors.backgroundLight};
  background-image: url(${(props) => props.src});
  background-position: center;
  background-repeat: no-repeat;
  background-size: cover;
`;

export { BlogPosts, BlogThumbnail, StyledBlog, StyledBlogPostCard };
