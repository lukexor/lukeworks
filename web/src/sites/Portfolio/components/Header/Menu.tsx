import React, { useCallback, useState } from "react";
import copy from "sites/Portfolio/data/copy.json";
import Icons from "sites/Portfolio/Icons";
import routes from "sites/Portfolio/routes.json";
import useEventListener from "util/hooks/useEventListener";
import { MenuLink, StyledMenu, StyledMenuIcon } from "./menu.styles";

type Routes = {
  [title: string]: string;
};

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
      {copy.Menu.list.map((menuItem, i) => (
        <MenuLink
          key={menuItem}
          to={(routes as Routes)[menuItem]}
          active={active === i}
          onClick={close}
          smooth
        >
          {menuItem}
        </MenuLink>
      ))}
    </StyledMenu>
  );
};

export default Menu;
export { MenuIcon };
