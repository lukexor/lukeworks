import styled from "styled-components";

const thumbWidth = "300px";
const thumbHeight = "250px";

const StyledProjects = styled.section``;

const ProjectPosts = styled.section`
  display: flex;
  flex-wrap: wrap;
`;

const StyledProjectPostCard = styled.article`
  width: ${thumbWidth};
  display: flex;
  flex-direction: column;

  h3 {
    font-size: ${(props) => props.theme.sizes.large};
  }

  a {
    color: ${(props) => props.theme.colors.primary} !important;

    &:hover {
      color: ${(props) => props.theme.colors.accentDark} !important;
    }
  }
`;

const ProjectThumbnail = styled.div<{ title: string; src: Maybe<string> }>`
  width: ${thumbWidth};
  height: ${thumbHeight};
  background-color: ${(props) => props.theme.colors.backgroundLight};
  background-image: url(${(props) => props.src});
  background-position: top left;
  background-repeat: no-repeat;
  background-size: cover;

  &::before {
    content: "${(props) => props.title}";
    display: ${(props) => (props.src ? "none" : "flex")};
    height: ${thumbHeight};
    justify-content: center;
    align-items: center;
  }
`;

export {
  ProjectPosts,
  ProjectThumbnail,
  StyledProjects,
  StyledProjectPostCard,
};
