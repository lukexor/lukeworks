import React, { useRef, useState } from "react";
import useClickOutside from "util/hooks/useClickOutside";
import features from "../../data/features.json";
import { HeaderBar, HeaderIcons, StyledHeader } from "./header.styles";
import Logo from "./Logo";
import Menu, { MenuIcon } from "./Menu";
import Search from "./Search";

const Header: React.FC = () => {
  const [menuVisible, setMenuVisible] = useState(false);
  const menuRef = useRef(null);
  useClickOutside(menuRef, () => setMenuVisible(false));

  const toggleMenu = () => setMenuVisible((visible) => !visible);

  return (
    <StyledHeader>
      <HeaderBar>
        <Logo />
        <HeaderIcons>
          {features.search && <Search />}
          <MenuIcon onClick={toggleMenu} />
        </HeaderIcons>
      </HeaderBar>
      <section ref={menuRef}>
        <Menu visible={menuVisible} close={() => setMenuVisible(false)} />
      </section>
    </StyledHeader>
  );
};

export default Header;
