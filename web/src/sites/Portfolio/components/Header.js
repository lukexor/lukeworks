import React, { useEffect, useRef, useState } from "react";

import { HeaderBar, HeaderIcons, StyledHeader } from "./header.styles";
import Logo from "./Logo";
import Menu, { MenuIcon } from "./Menu";
import Search from "./Search";

const Header = () => {
  const [menuVisible, setMenuVisible] = useState(false);
  const [searchVisible, setSearchVisible] = useState(false);
  const menuRef = useRef(null);
  const searchRef = useRef(null);

  const toggleMenu = () => setMenuVisible(!menuVisible);
  const clickSearch = () => {
    if (searchVisible) {
      // TODO: Add search functionality and onEnter listener
      setSearchVisible(false);
    } else {
      setSearchVisible(true);
    }
  };

  useEffect(() => {
    const clickOutside = (ref, cond, action, evt) => {
      const curr = ref.current;
      if (cond && curr && !curr.contains(evt.target)) {
        action(false);
      }
    };
    const clickOutsideMenu = (evt) =>
      clickOutside(menuRef, menuVisible, setMenuVisible, evt);
    const clickOutsideSearch = (evt) =>
      clickOutside(searchRef, searchVisible, setSearchVisible, evt);
    document.addEventListener("click", clickOutsideMenu);
    document.addEventListener("click", clickOutsideSearch);
    return () => {
      document.removeEventListener("click", clickOutsideMenu);
      document.removeEventListener("click", clickOutsideSearch);
    };
  });

  return (
    <StyledHeader>
      <HeaderBar>
        <Logo />
        <HeaderIcons>
          <span ref={searchRef}>
            <Search visible={searchVisible} onClick={clickSearch}></Search>
          </span>
          <MenuIcon onClick={toggleMenu} />
        </HeaderIcons>
      </HeaderBar>
      <span ref={menuRef}>
        <Menu visible={menuVisible} close={() => setMenuVisible(false)}></Menu>
      </span>
    </StyledHeader>
  );
};

export default Header;
