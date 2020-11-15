import { HashLink } from "react-router-hash-link";
import styled from "styled-components";

const Splash = styled.main`
  display: flex;
  flex-direction: column;
  flex: 1;
  align-content: center;
  width: 100%;
  min-height: 93vh;

  canvas {
    position: absolute;
    width: 100%;
    height: 100%;
    z-index: -999;
  }
`;

const Intro = styled.div`
  flex: 1;
  padding: 50px 0;

  @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
    padding: 100px 0;
  }
`;

const Title = styled.h1`
  color: ${(props) => props.theme.colors.primary};
  font-family: ${(props) => props.theme.fontTitle};
  font-size: ${(props) => props.theme.sizes.xlarge};
  font-weight: normal;
  text-align: center;

  @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
    font-size: ${(props) => props.theme.sizes.xxlarge};
  }
`;

const SubTitle = styled.h2`
  font-family: ${(props) => props.theme.fontTitle};
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
  width: 100%;
  padding: ${(props) => props.theme.sizes.medLarge} 0;
`;

const ExploreText = styled.h3`
  font-family: ${(props) => props.theme.fontBody};
  font-size: ${(props) => props.theme.sizes.med};
  font-weight: normal;

  @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
    font-size: ${(props) => props.theme.sizes.medlarge};
  }
`;

const ExploreLink = styled(HashLink)`
  border-radius: ${(props) => props.theme.sizes.large};
  border: 1px solid ${(props) => props.theme.colors.accentLight};
  color: ${(props) => props.theme.colors.accentLight};

  font-size: 36px;
  text-decoration: none;
`;

export {
  ExploreLink,
  ExploreText,
  Intro,
  Name,
  Explore,
  Splash,
  SubTitle,
  Title,
};
