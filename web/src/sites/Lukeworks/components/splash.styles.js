import styled from "styled-components";
import theme from "../theme";

const StyledSplash = styled.main`
  display: flex;
  flex-direction: column;
  flex: 1;
  align-content: center;
  width: 100%;
`;

const Intro = styled.div`
  flex: 1;
  padding: 50px 0;

  @media (min-width: ${theme.breakpoints.tablet}) {
    padding: 100px 0;
  }
`;

const Title = styled.h1`
  color: ${theme.colors.primary};
  font-family: ${theme.fontTitle};
  font-size: ${theme.sizes.xlarge};
  font-weight: normal;
  text-align: center;

  @media (min-width: ${theme.breakpoints.tablet}) {
    font-size: ${theme.sizes.xxlarge};
  }
`;

const SubTitle = styled.h2`
  font-family: ${theme.fontTitle};
  font-size: ${theme.sizes.large};
  font-weight: normal;
  text-align: center;

  @media (min-width: ${theme.breakpoints.tablet}) {
    font-size: ${theme.sizes.xlarge};
  }
`;

const Name = styled.span`
  color: ${theme.colors.secondary};
`;

const StyledExplore = styled.div`
  text-align: center;
  width: 100%;
  padding: ${theme.sizes.medLarge} 0;
`;

const ExploreText = styled.h3`
  font-family: ${theme.fontBody};
  font-size: ${theme.sizes.med};
  font-weight: normal;

  @media (min-width: ${theme.breakpoints.tablet}) {
    font-size: ${theme.sizes.medlarge};
  }
`;

const ExploreIcon = styled.span`
  border-radius: ${theme.sizes.large};
  border: 1px solid ${theme.colors.light};
  color: ${theme.colors.light};
  cursor: pointer;
  font-size: 36px;
`;

export {
  ExploreIcon,
  ExploreText,
  Intro,
  Name,
  StyledExplore,
  StyledSplash,
  SubTitle,
  Title,
};
