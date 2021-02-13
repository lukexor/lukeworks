import React, { SyntheticEvent, useEffect, useRef, useState } from "react";
import useClickOutside from "util/hooks/useClickOutside";
import { useDebounce } from "util/hooks/useDebounce";

import { copy } from "../../util/constants";
import {
  SearchBox,
  StyledClearIcon,
  StyledSearch,
  StyledSearchIcon,
} from "./search.styles";

type SearchIconProps = {
  onClick: () => void;
};

type ClearIconProps = {
  visible: boolean;
  onClick: () => void;
};

const SearchIcon: React.FC<SearchIconProps> = ({ onClick }) => (
  <StyledSearchIcon
    icon={copy.Search.icon}
    onClick={onClick}
  />
);

const ClearIcon: React.FC<ClearIconProps> = ({
  visible,
  onClick,
}) => (
  <StyledClearIcon
    visible={visible}
    icon={copy.Search.clearIcon}
    onClick={onClick}
  />
);

const Search: React.FC = () => {
  const [visible, setVisible] = useState(false);
  const searchRef = useRef(null);
  const [value, setValue] = useState("");
  useClickOutside(searchRef, () => setVisible(false));

  const debouncedSearch = useDebounce(value, 500);

  const clickIcon = () => {
    if (visible) {
      setValue("");
      setVisible(false);
    } else {
      setVisible(true);
    }
  };

  const handleChange = (evt: SyntheticEvent<HTMLInputElement>) => {
    setValue((evt.target as HTMLInputElement).value);
  };

  useEffect(() => {
    if (debouncedSearch) {
      // TODO: Add search functionality withpop menu
    }
  }, [debouncedSearch]);

  return (
    <StyledSearch ref={searchRef}>
      <SearchBox visible={visible}>
        <SearchIcon onClick={clickIcon} />
        <input
          type="text"
          name="search"
          id="search"
          placeholder={copy.Search.placeholder}
          value={value}
          onChange={handleChange}
        />
        <ClearIcon
          visible={visible && value.length > 0}
          onClick={() => setValue("")}
        />
      </SearchBox>
    </StyledSearch>
  );
};

export default Search;
export { SearchIcon };
