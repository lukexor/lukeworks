import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";
import styled, { css } from "styled-components";

const StyledSearchIcon = styled(FontAwesomeIcon)`
  position: relative;
  margin: 0 10px;
  z-index: 1;
  cursor: pointer;
  color: ${(props) => props.theme.colors.accentLight};
  font-size: ${(props) => props.theme.sizes.medLarge};
  transition: color 0.3s ease;

  &:hover {
    color: ${(props) => props.theme.colors.accentDark};
  }
`;

// Neeed to filter out invalid HTML props
const StyledClearIcon = styled(({ visible: _, ...props }) => (
  <FontAwesomeIcon {...props} />
))`
  cursor: pointer;
  display: ${(props) => (props.visible ? "block" : "none")};
  color: ${(props) => props.theme.colors.secondary};
  font-size: ${(props) => props.theme.sizes.medLarge};
  transition: color 0.3s ease;

  &:hover {
    color: ${(props) => props.theme.colors.accentDark};
  }
`;

const StyledSearch = styled.div`
  display: flex;
  align-items: center;
  height: 100%;
`;

// Neeed to filter out invalid HTML props
const SearchBox = styled(({ visible: _, ...props }) => <div {...props} />)`
  display: flex;
  align-items: center;
  height: ${(props) => props.theme.sizes.xlarge};
  ${(props) =>
    props.visible
      ? css`
          margin: 0 10px;
          border-radius: ${(props) => props.theme.sizes.large};
          border: 1px solid ${(props) => props.theme.colors.accentDark};
          background: ${(props) => props.theme.colors.backgroundLight};
          padding-right: 10px;
        `
      : null}

  input {
    width: ${(props) => (props.visible ? "200px" : 0)};
    height: ${(props) => props.theme.sizes.large};
    padding: 0;
    border: none;
    color: ${(props) => props.theme.colors.primary};
    caret-color: ${(props) => props.theme.colors.primary};
    background: ${(props) => props.theme.colors.backgroundLight};
    outline: none;
    transition: width 0.3s ease-out;

    &:-webkit-autofill {
      -webkit-text-fill-color: ${(props) => props.theme.colors.primary};
      -webkit-box-shadow: 0 0 0px 1000px
        ${(props) => props.theme.colors.backgroundLight} inset;
    }

    &::placeholder {
      color: ${(props) => props.theme.colors.accentDark};
      opacity: 1;
    }
  }
`;

export { SearchBox, StyledClearIcon, StyledSearch, StyledSearchIcon };
