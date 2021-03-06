import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import styled from "styled-components";

const StyledSection = styled.section`
  min-height: calc(100vh - ${(props) => props.theme.sizes.xxlarge});
`;

const Splash = styled(StyledSection)`
  display: flex;
  flex-direction: column;
  flex: 1;
  align-content: center;

  canvas {
    position: absolute;
    z-index: -1000;
  }
`;

const Heading = styled.div`
  flex: 1;
  padding: 50px 0;

  &::before {
    content: "";
    display: block;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    background: linear-gradient(
      rgba(18, 16, 16, 0) 50%,
      rgba(0, 0, 0, 0.25) 50%
    );
    z-index: -2;
    background-size: 100% 3px, 4px 100%;
    pointer-events: none;
  }

  @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
    padding: 100px 0;
  }
`;

const Subtitle = styled.p`
  font-family: ${(props) => props.theme.fontSerif};
  font-size: ${(props) => props.theme.sizes.large};
  font-weight: normal;
  line-height: 1.6em;
  text-align: center;

  @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
    font-size: ${(props) => props.theme.sizes.xlarge};
  }
`;

const Name = styled.span`
  color: ${(props) => props.theme.colors.accentDark};
`;

const Explore = styled.div`
  text-align: center;
  padding: ${(props) => props.theme.sizes.medLarge} 0;
  margin-bottom: 20px;
`;

const ExploreText = styled.p`
  font-size: ${(props) => props.theme.sizes.med};

  @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
    font-size: ${(props) => props.theme.sizes.medLarge};
  }
`;

const ExploreIcon = styled(FontAwesomeIcon)`
  color: ${(props) => props.theme.colors.accentLight};
  font-size: ${(props) => props.theme.sizes.xxlarge};
  text-decoration: none;
  transition: color 0.5s;

  &:hover {
    color: ${(props) => props.theme.colors.accentDark};
  }
`;

export {
  Explore,
  ExploreIcon,
  ExploreText,
  Heading,
  Subtitle,
  Name,
  Splash,
  StyledSection,
};
