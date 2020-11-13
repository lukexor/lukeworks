import React from "react";
import styled from "styled-components";
import theme from "../theme";

const StyledMenu = styled.span`
  color: ${theme.colors.light};
  cursor: pointer;
  font-size: ${theme.sizes.xlarge};
  margin: auto 0;
`;

const Menu = () => (
  <StyledMenu className="material-icons" alt="navigation menu">
    menu
  </StyledMenu>
);

export default Menu;
