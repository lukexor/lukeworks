import Logo from "./Logo";
import Menu from "./Menu";
import React from "react";
import styled from "styled-components";
import theme from "../theme";

const StyledHeader = styled.header`
  display: flex;
  justify-content: space-between;
  background: ${theme.colors.background};
  padding: ${theme.sizes.small} ${theme.sizes.med};
  position: sticky;
  top: 0;
`;

const Header = () => (
  <StyledHeader>
    <Logo />
    <Menu />
  </StyledHeader>
);

export default Header;
