import { HashLink } from "react-router-hash-link";
import styled from "styled-components";

const StyledLogo = styled(HashLink)`
  color: ${(props) => props.theme.colors.primary};
  font-family: ${(props) => props.theme.fontSerif};
  font-size: ${(props) => props.theme.sizes.large};
  text-shadow: 1px 1px 2px ${(props) => props.theme.colors.accentDark};
  text-decoration: none;

  &:hover {
    text-shadow: 0 0px 4px ${(props) => props.theme.colors.accentLight};
  }

  @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
    font-size: ${(props) => props.theme.sizes.xlarge};
  }
`;

export { StyledLogo };
