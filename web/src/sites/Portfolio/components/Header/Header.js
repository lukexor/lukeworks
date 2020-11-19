import React, { useEffect, useRef, useState } from "react";

import { HeaderBar, HeaderIcons, StyledHeader } from "./header.styles";
import Logo from "./Logo";
import Menu, { MenuIcon } from "./Menu";
import Search from "./Search";

const Header = () => {
  const [menuVisible, setMenuVisible] = useState(false);
  const menuRef = useRef(null);

  const toggleMenu = () => setMenuVisible(!menuVisible);

  useEffect(() => {
    const clickOutside = (ref, cond, action, evt) => {
      const curr = ref.current;
      if (cond && curr && !curr.contains(evt.target)) {
        action(false);
      }
    };
    const clickOutsideMenu = (evt) =>
      clickOutside(menuRef, menuVisible, setMenuVisible, evt);
    document.addEventListener("click", clickOutsideMenu);
    return () => {
      document.removeEventListener("click", clickOutsideMenu);
    };
  });

  return (
    <StyledHeader>
      <HeaderBar>
        <Logo />
        <HeaderIcons>
          <Search />
          <MenuIcon onClick={toggleMenu} />
        </HeaderIcons>
      </HeaderBar>
      <span ref={menuRef}>
        <Menu visible={menuVisible} close={() => setMenuVisible(false)} />
      </span>
    </StyledHeader>
  );
};

export default Header;
