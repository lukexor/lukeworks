import clsx from "clsx";
import { ClearIcon, SearchIcon } from "components/icons";
import LinkIcon from "components/linkIcon";
import copy from "data/layout.json";
import useClickOutside from "hooks/useClickOutside";
import { useDebounce } from "hooks/useDebounce";
import { SyntheticEvent, useEffect, useRef, useState } from "react";
import s from "./search.module.css";

export default function SearchBox() {
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
    <div className={s.wrapper} ref={wrapperRef}>
      <div className={clsx(s.searchBox, expanded && s.expanded)}>
        <LinkIcon
          className={s.searchIcon}
          icon={SearchIcon}
          onClick={toggleExpanded}
        />
        <input
          type="text"
          name="search"
          id="search"
          ref={searchRef}
          placeholder={copy.search_placeholder}
          value={value}
          onChange={handleChange}
          onFocus={() => setExpanded(true)}
          onBlur={() => setExpanded(false)}
        />
        <LinkIcon
          className={clsx(s.clearIcon, expanded && s.visible)}
          icon={ClearIcon}
          onClick={() => setValue("")}
        />
      </div>
    </div>
  );
}
