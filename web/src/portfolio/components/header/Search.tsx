import "./Search.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import classnames from "classnames";
import useClickOutside from "hooks/useClickOutside";
import { useDebounce } from "hooks/useDebounce";
import Icons from "portfolio/Icons";
import { SyntheticEvent, useEffect, useRef, useState } from "react";
import copy from "../../data/copy.json";

type SearchIconProps = {
  onClick: () => void;
};

type ClearIconProps = {
  visible: boolean;
  onClick: () => void;
};

const SearchIcon = ({ onClick }: SearchIconProps) => (
  <FontAwesomeIcon
    className="link-icon search-icon"
    icon={Icons.search}
    onClick={onClick}
  />
);

const ClearIcon = ({ visible, onClick }: ClearIconProps) => {
  return (
    <FontAwesomeIcon
      className={classnames({ "link-icon": true, "clear-icon": true, visible })}
      icon={Icons.clearField}
      onClick={onClick}
    />
  );
};

const SearchBox = () => {
  const [expanded, setExpanded] = useState(false);
  const [value, setValue] = useState("");
  const debouncedSearch = useDebounce(value, 300);
  const wrapperRef = useRef<HTMLDivElement>(null);
  const searchRef = useRef<HTMLInputElement>(null);

  useClickOutside(wrapperRef, () => setExpanded(false));

  const toggleExpanded = () => {
    setValue("");
    setExpanded((expanded) => {
      if (!expanded) {
        searchRef.current?.focus();
      }
      return !expanded;
    });
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
    <div className="search-wrapper" ref={wrapperRef}>
      <div className={classnames({ "search-box": true, expanded })}>
        <SearchIcon onClick={toggleExpanded} />
        <input
          type="text"
          name="search"
          id="search"
          ref={searchRef}
          placeholder={copy.Search.placeholder}
          value={value}
          onChange={handleChange}
          onFocus={() => setExpanded(true)}
          onBlur={() => setExpanded(false)}
        />
        <ClearIcon
          visible={expanded && value.length > 0}
          onClick={() => setValue("")}
        />
      </div>
    </div>
  );
};

export default SearchBox;
export { SearchIcon };
