import React from "react";
import SitePaths from "sitePaths";
import { scrollWithHeaderOffset } from "util/scroll";

import { copy } from "../util/constants";
import { StyledLogo } from "./logo.styles";

const Logo = () => (
  <StyledLogo
    to={SitePaths.home}
    alt={copy.Logo.alt}
    scroll={scrollWithHeaderOffset}
  >
    {copy.Logo.text}
  </StyledLogo>
);

export default Logo;
