import React from "react";

import { StyledHeader } from "./header.styles";
import Logo from "./Logo";
import Menu from "./Menu";

const Header = () => (
  <StyledHeader>
    <Logo />
    <Menu />
  </StyledHeader>
);

export default Header;
