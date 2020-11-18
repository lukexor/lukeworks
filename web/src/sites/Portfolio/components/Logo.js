import React from "react";
import SitePaths from "sitePaths";

import { copy } from "../util/constants";
import { StyledLogo } from "./logo.styles";

const Logo = () => (
  <StyledLogo smooth to={SitePaths.home} alt={copy.Logo.alt}>
    {copy.Logo.text}
  </StyledLogo>
);

export default Logo;
