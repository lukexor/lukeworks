import React from "react";
import copy from "../../data/copy.json";
import routes from "../../routes.json";
import { StyledLogo } from "./logo.styles";

const Logo: React.FC = () => (
  <StyledLogo smooth to={routes.Home} className="img">
    {copy.Logo.text}
  </StyledLogo>
);

export default Logo;
