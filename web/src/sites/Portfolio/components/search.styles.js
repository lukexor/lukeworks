import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import styled from "styled-components";

const StyledSearchIcon = styled(FontAwesomeIcon)`
  position: relative;
  margin: 0 10px;
  z-index: 1;
  cursor: pointer;
  color: ${(props) => props.theme.colors.accentLight};
  font-size: ${(props) => props.theme.sizes.medLarge};
  transition: color 0.5s ease;

  &:hover {
    color: ${(props) => props.theme.colors.accentDark};
  }

  &.visible {
  }
`;

const StyledSearch = styled.div`
  display: flex;
  align-items: center;
  height: 100%;
`;

const SearchBox = styled.div`
  display: flex;
  align-items: center;
  height: ${(props) => props.theme.sizes.xlarge};

  &.visible {
    border-radius: ${(props) => props.theme.sizes.large};
    border: 1px solid ${(props) => props.theme.colors.accentDark};
    background: ${(props) => props.theme.colors.backgroundLight};
    padding-right: 10px;
  }

  input {
    -webkit-text-fill-color: ${(props) => props.theme.colors.primary};
    -webkit-box-shadow: 0 0 0px 1000px
      ${(props) => props.theme.colors.backgroundLight} inset;
    width: 0;
    height: ${(props) => props.theme.sizes.large};
    padding: 0;
    border: none;
    color: ${(props) => props.theme.colors.primary};
    caret-color: ${(props) => props.theme.colors.primary};
    background: ${(props) => props.theme.colors.background};
    outline: none;
    transition: width 0.3s ease-out;
  }

  input.visible {
    width: 200px;
  }
`;

export { SearchBox, StyledSearch, StyledSearchIcon };
