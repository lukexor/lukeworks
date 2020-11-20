import PropTypes from "prop-types";
import React, { useCallback, useState } from "react";
import useEventListener from "util/hooks/useEventListener";

import { copy } from "../../util/constants";
import { MenuLink, StyledMenu, StyledMenuIcon } from "./menu.styles";

const MenuIcon = (props) => (
  <StyledMenuIcon alt={copy.Menu.alt} icon={copy.Menu.icon} {...props} />
);

const Menu = ({ visible, close }) => {
  const [active, setActive] = useState(0);

  const updateActive = useCallback(() => {
    const offsets = Array.from(document.querySelectorAll("span.anchor"))
      .concat(document.querySelector("section#splash"))
      .map((el) => el.offsetTop)
      .sort((a, b) => a - b);

    let closest = active;
    let closestDiff = Infinity;
    offsets.forEach((offset, i) => {
      const diff = Math.abs(window.scrollY - offset);
      if (diff < closestDiff) {
        closestDiff = diff;
        closest = i;
      }
    });
    setActive(closest);
  }, []);

  useEventListener("scroll", updateActive);

  return (
    <StyledMenu visible={visible}>
      {copy.Menu.links.map(([link, text], i) => (
        <MenuLink
          key={link}
          to={link}
          active={active === i}
          onClick={close}
          smooth
        >
          {text}
        </MenuLink>
      ))}
    </StyledMenu>
  );
};

Menu.propTypes = {
  visible: PropTypes.bool,
  close: PropTypes.func.isRequired,
};

export default Menu;
export { MenuIcon };
