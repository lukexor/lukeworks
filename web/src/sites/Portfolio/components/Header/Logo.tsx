import React from "react";
import copy from "../../data/copy.json";
import routes from "../../routes";
import { StyledLogo } from "./logo.styles";

const Logo: React.FC = () => (
  <StyledLogo smooth to={routes.home.path}>
    {copy.Logo.text}
  </StyledLogo>
);

export default Logo;
