// import { SyntheticEvent, useEffect, useRef, useState } from "react";
// import Icons from "portfolio/Icons";
// import useClickOutside from "hooks/useClickOutside";
// import { useDebounce } from "hooks/useDebounce";
// import copy from "../../data/copy.json";

// type SearchIconProps = {
//   onClick: () => void;
// };

// type ClearIconProps = {
//   visible: boolean;
//   onClick: () => void;
// };

// const SearchIcon = ({ onClick }) => (
//   <StyledSearchIcon icon={Icons.search} onClick={onClick} />
// );

// const ClearIcon = ({ visible, onClick }) => (
//   <StyledClearIcon
//     visible={visible}
//     icon={Icons.clearField}
//     onClick={onClick}
//   />
// );

// const Search = () => {
//   const [open, setOpen] = useState(false);
//   const searchRef = useRef(null);
//   const [value, setValue] = useState("");
//   useClickOutside(searchRef, () => setOpen(false));

//   const debouncedSearch = useDebounce(value, 500);

//   const toggleOpen = () => {
//     setValue("");
//     setOpen((open) => !open);
//   };

//   const handleChange = (evt: SyntheticEvent<HTMLInputElement>) => {
//     setValue((evt.target as HTMLInputElement).value);
//   };

//   useEffect(() => {
//     if (debouncedSearch) {
//       // TODO: Add search functionality withpop menu
//     }
//   }, [debouncedSearch]);

//   return (
//     <StyledSearch ref={searchRef}>
//       <SearchBox visible={open}>
//         <SearchIcon onClick={toggleOpen} />
//         <input
//           type="text"
//           name="search"
//           id="search"
//           placeholder={copy.Search.placeholder}
//           value={value}
//           onChange={handleChange}
//           onFocus={() => setOpen(true)}
//           onBlur={toggleOpen}
//         />
//         <ClearIcon
//           visible={open && value.length > 0}
//           onClick={() => setValue("")}
//         />
//       </SearchBox>
//     </StyledSearch>
//   );
// };
const Search = () => {
  return <></>;
};
const SearchIcon = () => null;

export default Search;
export { SearchIcon };
