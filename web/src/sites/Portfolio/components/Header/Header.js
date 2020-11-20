import React, { useCallback, useRef, useState } from "react";
import useEventListener from "util/hooks/useEventListener";

import { HeaderBar, HeaderIcons, StyledHeader } from "./header.styles";
import Logo from "./Logo";
import Menu, { MenuIcon } from "./Menu";
import Search from "./Search";

const Header = () => {
  const [menuVisible, setMenuVisible] = useState(false);
  const menuRef = useRef(null);

  const toggleMenu = () => setMenuVisible(!menuVisible);

  const clickOutsideMenu = useCallback(
    (event) => {
      const menu = menuRef.current;
      if (menuVisible && menu && !menu.contains(event.target)) {
        setMenuVisible(false);
      }
    },
    [menuVisible, menuRef]
  );

  useEventListener("click", clickOutsideMenu);

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
