import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { HashLink } from "react-router-hash-link";
import styled from "styled-components";

const StyledMenuIcon = styled(FontAwesomeIcon)`
  color: ${(props) => props.theme.colors.accentLight};
  cursor: pointer;
  font-size: ${(props) => props.theme.sizes.xlarge};
  margin: auto 10px;
  transition: color 0.5s ease;

  :hover {
    color: ${(props) => props.theme.colors.accentDark};
  }
`;

const StyledMenu = styled.nav`
  background-color: ${(props) => props.theme.colors.background};
  position: absolute;
  width: 100%;
  height: 0;
  left: 0;
  overflow: hidden;
  text-align: center;
  font-family: ${(props) => props.theme.fontSerif};
  line-height: ${(props) => props.theme.sizes.xlarge};
  transition: height 0.3s ease-out;

  &.visible {
    height: 13ch;
  }

  @media (min-width: ${(props) => props.theme.breakpoints.desktopSmall}) {
    width: 15ch;
    left: auto;
    right: 1ch;
  }
`;

const MenuLink = styled(HashLink)`
  display: block;
  color: ${(props) => props.theme.colors.secondary};
  text-decoration: none;
  text-transform: uppercase;
  transition: color 0.5s;

  &.active {
    color: ${(props) => props.theme.colors.primary};
  }

  :hover {
    color: ${(props) => props.theme.colors.accentDark};
  }
`;

export { StyledMenu, StyledMenuIcon, MenuLink };
