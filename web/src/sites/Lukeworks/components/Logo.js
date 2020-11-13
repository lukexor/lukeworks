import { Link } from "react-router-dom";
import React from "react";
import SitePaths from "sitePaths";
import styled from "styled-components";
import theme from "../theme";

const StyledLogo = styled(Link)`
  color: ${theme.colors.primary};
  font-family: ${theme.fontTitle};
  font-size: ${theme.sizes.large};
  text-shadow: 2px -2px 1px ${theme.colors.secondary},
    0 0px 2px ${theme.colors.light};

  @media (min-width: ${theme.breakpoints.tablet}) {
    font-size: ${theme.sizes.xlarge};
  }
`;

const Logo = () => (
  <StyledLogo to={SitePaths.home} alt="homepage">
    LP
  </StyledLogo>
);

export default Logo;
