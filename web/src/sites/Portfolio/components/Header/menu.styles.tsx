import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { HashLink } from "react-router-hash-link";
import styled from "styled-components";

const StyledMenuIcon = styled(FontAwesomeIcon)`
  color: ${(props) => props.theme.colors.accentLight};
  cursor: pointer;
  font-size: ${(props) => props.theme.sizes.xlarge};
  margin: auto 10px;
  transition: color 0.5s ease;

  &:hover {
    color: ${(props) => props.theme.colors.accentDark};
  }
`;

const StyledMenu = styled.nav<{ visible: boolean }>`
  background-color: ${(props) => props.theme.colors.backgroundLight};
  box-shadow: 0 2px 0 ${(props) => props.theme.colors.accentDark};
  position: absolute;
  height: ${(props) => (props.visible ? "13ch" : 0)};
  width: 15ch;
  left: auto;
  right: 1ch;
  overflow: hidden;
  text-align: center;
  font-family: ${(props) => props.theme.fontSerif};
  line-height: ${(props) => props.theme.sizes.xlarge};
  transition: height 0.3s ease-out;
`;

// Need to filter out invalid HTML props
const MenuLink = styled(({ active: _, ...props }) => <HashLink {...props} />)`
  display: block;
  color: ${(props) =>
    props.active
      ? props.theme.colors.primary
      : props.theme.colors.secondary}!important;
  text-decoration: none;
  text-transform: uppercase;
  transition: color 0s;

  &:hover {
    color: ${(props) => props.theme.colors.accentDark}!important;
  }
`;

export { StyledMenu, StyledMenuIcon, MenuLink };
