import { Link } from "react-router-dom";
import styled from "styled-components";

const StyledLogo = styled(Link)`
  color: ${(props) => props.theme.colors.primary};
  font-family: ${(props) => props.theme.fontTitle};
  font-size: ${(props) => props.theme.sizes.large};
  text-shadow: 2px -2px 2px ${(props) => props.theme.colors.background},
    0 0px 4px ${(props) => props.theme.colors.accentDark};

  @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
    font-size: ${(props) => props.theme.sizes.xlarge};
  }
`;

export { StyledLogo };
