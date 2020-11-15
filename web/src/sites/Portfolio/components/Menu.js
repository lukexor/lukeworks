import React from "react";
import { copy } from "util/constants";

import { StyledMenu } from "./menu.styles";

const Menu = () => (
  <StyledMenu className="material-icons" alt={copy.Menu.alt}>
    {copy.Menu.logo}
  </StyledMenu>
);

export default Menu;
