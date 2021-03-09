import React, { useCallback, useState } from "react";
import copy from "sites/Portfolio/data/copy.json";
import Icons from "sites/Portfolio/Icons";
import routes from "sites/Portfolio/routes.json";
import useEventListener from "util/hooks/useEventListener";
import { MenuLink, StyledMenu, StyledMenuIcon } from "./menu.styles";

type MenuIconProps = {
  onClick: () => void;
};
type MenuProps = {
  visible: boolean;
  close: () => void;
};

type Route = keyof typeof routes;
type MenuItem = {
  url: Route;
  title: string;
};

const routeLinks: MenuItem[] = copy.Menu.list.map((item) => ({
  title: item,
  url: item.toLowerCase() as Route,
}));

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
      {routeLinks.map(({ url, title }, i) => (
        <MenuLink
          key={url}
          to={routes[url]}
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
