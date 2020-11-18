import React from "react";
import SitePaths from "sitePaths";

import { copy } from "../util/constants";
import {
  BackToTop,
  BackToTopIcon,
  Copyright,
  FooterBar,
  Name,
  SocialIcon,
  SocialIcons,
  StyledFooter,
} from "./footer.styles";

const Footer = () => (
  <StyledFooter>
    <FooterBar>
      <BackToTop smooth to={SitePaths.home} alt={copy.Footer.backToTop.alt}>
        <BackToTopIcon icon={copy.Footer.backToTop.icon} />
      </BackToTop>
      <SocialIcons>
        {copy.Footer.socialIcons.map(([icon, link, alt]) => (
          <a key={link} href={link} alt={alt}>
            <SocialIcon icon={icon} />
          </a>
        ))}
      </SocialIcons>
      <Copyright>
        {copy.Footer.copyright}{" "}
        <Name>
          {copy.Contact.firstName} {copy.Contact.lastName}
        </Name>{" "}
        {copy.Footer.rightsReserved}
      </Copyright>
    </FooterBar>
  </StyledFooter>
);

export default Footer;
