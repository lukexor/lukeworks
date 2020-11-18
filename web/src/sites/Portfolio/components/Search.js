import PropTypes from "prop-types";
import React from "react";

import { copy } from "../util/constants";
import { SearchBox, StyledSearch, StyledSearchIcon } from "./search.styles";

const SearchIcon = ({ visible, ...props }) => (
  <StyledSearchIcon
    className={visible ? "visible" : null}
    alt={copy.Search.alt}
    icon={copy.Search.icon}
    {...props}
  />
);

SearchIcon.propTypes = {
  visible: PropTypes.bool,
};

const Search = (props) => {
  return (
    <StyledSearch>
      <SearchBox className={props.visible ? "visible" : null}>
        <SearchIcon {...props} />
        <input
          className={props.visible ? "visible" : null}
          type="text"
          name="search"
          id="search"
          placeholder={copy.Search.alt}
        />
        {/* TODO: Add X to clear contents and add value state */}
      </SearchBox>
    </StyledSearch>
  );
};

Search.propTypes = {
  visible: PropTypes.bool,
};

export default Search;
export { SearchIcon };
