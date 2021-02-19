import React from "react";
import copy from "../../data/copy.json";
import Icons from "../../Icons";
import routes from "../../routes.json";
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

const Footer: React.FC = () => (
  <StyledFooter>
    <FooterBar>
      <BackToTop smooth to={routes.Home} className="img">
        <BackToTopIcon icon={Icons.backToTop} />
      </BackToTop>
      <SocialIcons>
        {copy.Footer.socialIcons.map(({ icon, link, title }) => (
          <a key={link} href={link} title={title} className="img">
            <SocialIcon icon={Icons[icon]} />
          </a>
        ))}
      </SocialIcons>
      <Copyright>
        {copy.Footer.copyright} {new Date().getFullYear()}{" "}
        <Name>
          {copy.Contact.firstName} {copy.Contact.lastName}
        </Name>{" "}
        {copy.Footer.rightsReserved}
      </Copyright>
    </FooterBar>
  </StyledFooter>
);

export default Footer;
