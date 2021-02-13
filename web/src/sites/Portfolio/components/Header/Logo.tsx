import React from "react";
import SitePaths from "sitePaths";

import { copy } from "../../util/constants";
import { StyledLogo } from "./logo.styles";

const Logo: React.FC = () => (
  <StyledLogo smooth to={SitePaths.home}>
    {copy.Logo.text}
  </StyledLogo>
);

export default Logo;
