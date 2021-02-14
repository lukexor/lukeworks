import React, { useCallback, useState } from "react";
import copy from "sites/Portfolio/data/copy.json";
import Icons from "sites/Portfolio/Icons";
import useEventListener from "util/hooks/useEventListener";
import { MenuLink, StyledMenu, StyledMenuIcon } from "./menu.styles";

type MenuIconProps = {
  onClick: () => void;
};

type MenuProps = {
  visible: boolean;
  close: () => void;
};

const MenuIcon: React.FC<MenuIconProps> = (props) => (
  <StyledMenuIcon icon={Icons.menu} {...props} />
);

const Menu: React.FC<MenuProps> = ({ visible, close }) => {
  const [active, setActive] = useState(0);

  const updateActive = useCallback(() => {
    const offsets = Array.from(document.querySelectorAll("span.anchor"))
      .concat(document.querySelector("section#splash") || [])
      .map((el) => (el as HTMLElement).offsetTop)
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
      {copy.Menu.links.map(({ path, title }, i) => (
        <MenuLink
          key={path}
          to={path}
          active={active === i}
          onClick={close}
          smooth
        >
          {title}
        </MenuLink>
      ))}
    </StyledMenu>
  );
};

export default Menu;
export { MenuIcon };
