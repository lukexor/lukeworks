import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { HashLink } from "react-router-hash-link";
import styled from "styled-components";

const StyledFooter = styled.footer`
  position: relative;
  background: ${(props) => props.theme.colors.backgroundLight};
`;

const FooterBar = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
`;

const Copyright = styled.p`
  font-size: ${(props) => props.theme.sizes.medSmall};
  color: ${(props) => props.theme.colors.secondary};
`;

const Name = styled.span`
  font-weight: bold;
`;

const BackToTop = styled(HashLink)`
  position: absolute;
  top: -36px;
`;

const BackToTopIcon = styled(FontAwesomeIcon)`
  color: ${(props) => props.theme.colors.accentLight};
  font-size: ${(props) => props.theme.sizes.xlarge};
  padding: 15px;
  text-decoration: none;
  transition: color 0.5s;

  &:hover {
    color: ${(props) => props.theme.colors.accentDark};
  }
`;

const SocialIcons = styled.div`
  display: flex;
  justify-content: space-between;
`;

const SocialIcon = styled(FontAwesomeIcon)`
  color: ${(props) => props.theme.colors.accentLight};
  font-size: ${(props) => props.theme.sizes.xlarge};
  padding: 15px;
  text-decoration: none;
  transition: color 0.5s;

  &:hover {
    color: ${(props) => props.theme.colors.accentDark};
  }
`;

export {
  BackToTop,
  BackToTopIcon,
  Copyright,
  FooterBar,
  Name,
  SocialIcon,
  SocialIcons,
  StyledFooter,
};
