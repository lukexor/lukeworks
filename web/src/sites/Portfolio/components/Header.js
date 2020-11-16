import React, { useState } from "react";

import { HeaderBar, StyledHeader } from "./header.styles";
import Logo from "./Logo";
import Menu, { MenuIcon } from "./Menu";

const Header = () => {
  const [menuVisible, setMenuVisible] = useState(false);

  return (
    <StyledHeader>
      <HeaderBar>
        <Logo />
        <MenuIcon toggleVisible={() => setMenuVisible(!menuVisible)} />
      </HeaderBar>
      <Menu visible={menuVisible} close={() => setMenuVisible(false)}></Menu>
    </StyledHeader>
  );
};

export default Header;
