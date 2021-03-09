import { HashLink } from "react-router-hash-link";
import styled from "styled-components";

const StyledLogo = styled(HashLink)`
  color: ${(props) => props.theme.colors.primary};
  font-family: ${(props) => props.theme.fontSerif};
  font-size: ${(props) => props.theme.sizes.large};
  text-decoration: none;
  transition: color 0.5s ease;

  &:hover {
    color: ${(props) => props.theme.colors.accentDark};
  }

  @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
    font-size: ${(props) => props.theme.sizes.xlarge};
  }
`;

export { StyledLogo };
