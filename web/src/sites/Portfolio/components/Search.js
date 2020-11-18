import PropTypes from "prop-types";
import React, { useEffect, useState } from "react";
import { useDebounce } from "util/hooks/useDebounce";

import { copy } from "../util/constants";
import {
  SearchBox,
  StyledClearIcon,
  StyledSearch,
  StyledSearchIcon,
} from "./search.styles";

const SearchIcon = ({ visible, onClick }) => (
  <StyledSearchIcon
    visible={visible}
    alt={copy.Search.alt}
    icon={copy.Search.icon}
    onClick={onClick}
  />
);

SearchIcon.propTypes = {
  visible: PropTypes.bool,
  onClick: PropTypes.func.isRequired,
};

const ClearIcon = ({ visible, displayed, onClick }) => (
  <StyledClearIcon
    visible={visible}
    displayed={displayed}
    alt={copy.Search.cleareAlt}
    icon={copy.Search.clearIcon}
    onClick={onClick}
  />
);

ClearIcon.propTypes = {
  visible: PropTypes.bool,
  displayed: PropTypes.bool,
  onClick: PropTypes.func.isRequired,
};

const Search = () => {
  const [visible, setVisible] = useState(false);
  const [value, setValue] = useState("");

  const debouncedSearch = useDebounce(value, 500);

  const clickIcon = () => {
    if (visible) {
      setValue("");
      setVisible(false);
    } else {
      setVisible(true);
    }
  };

  const handleChange = (evt) => {
    setValue(evt.target.value);
  };

  useEffect(() => {
    if (debouncedSearch) {
      // TODO: Add search functionality withpop menu
    }
  }, [debouncedSearch]);

  return (
    <StyledSearch>
      <SearchBox visible={visible}>
        <SearchIcon visible={visible} onClick={clickIcon} />
        <input
          visible={visible}
          type="text"
          name="search"
          id="search"
          placeholder={copy.Search.alt}
          value={value}
          onChange={handleChange}
        />
        <ClearIcon
          visible={visible}
          displayed={value}
          onClick={() => setValue("")}
        />
      </SearchBox>
    </StyledSearch>
  );
};

export default Search;
export { SearchIcon };
