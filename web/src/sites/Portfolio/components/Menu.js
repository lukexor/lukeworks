import PropTypes from "prop-types";
import React, { useEffect, useState } from "react";

import { copy } from "../util/constants";
import { MenuLink, StyledMenu, StyledMenuIcon } from "./menu.styles";

const MenuIcon = (props) => (
  <StyledMenuIcon alt={copy.Menu.alt} icon={copy.Menu.icon} {...props} />
);

const Menu = ({ visible, close }) => {
  const [active, setActive] = useState(0);

  const updateActive = () => {
    const offsets = Array.from(document.querySelectorAll("a.anchor"))
      .concat(document.querySelector("a#home"))
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
    if (closest !== active) {
      setActive(closest);
    }
  };

  useEffect(() => {
    window.addEventListener("scroll", updateActive);
    return () => {
      window.removeEventListener("scroll", updateActive);
    };
  });

  return (
    <StyledMenu className={visible ? "visible" : null}>
      {copy.Menu.links.map(([link, text], i) => (
        <MenuLink
          key={link}
          to={link}
          className={active === i && "active"}
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
