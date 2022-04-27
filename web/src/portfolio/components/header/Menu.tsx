import "./Menu.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import classNames from "classnames";
import useClickOutside from "hooks/useClickOutside";
import useEventListener from "hooks/useEventListener";
import Icons from "portfolio/Icons";
import { useCallback, useEffect, useRef, useState } from "react";
import { Link, useLocation } from "react-router-dom";
import routes from "routes.json";

const Menu = () => {
  const [active, setActive] = useState(-1);
  const [open, setOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const menuIconRef = useRef<HTMLDivElement>(null);
  const toggleOpen = () => setOpen((open) => !open);
  const location = useLocation();

  useEffect(() => {
    const height = open ? "17.5ch" : "0";
    menuRef.current?.style.setProperty("--height", height);
  }, [open]);

  const updateActive = useCallback(() => {
    const offsets = Array.from(document.querySelectorAll("span.anchor"))
      .map((el) => (el as HTMLElement).offsetTop)
      .sort((a, b) => a - b);

    let closest = -1;
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

  useEffect(() => {
    updateActive();
  });

  useEventListener("scroll", updateActive);

  useClickOutside(menuRef, (event) => {
    const icon = menuIconRef.current;
    const target = event.target as Node;
    if (!icon || (icon && !icon.contains(target))) {
      setOpen(false);
    }
  });

  return (
    <>
      <div ref={menuIconRef}>
        <FontAwesomeIcon
          className="menu-icon ripple"
          icon={Icons.menu}
          onClick={toggleOpen}
        />
      </div>
      <nav className="menu" ref={menuRef}>
        {Object.values(routes.portfolio.sections).map(
          ({ path, hash, title }, i) => {
            const to = `${path}${hash}`;
            const props = {
              key: to,
              className: classNames({
                "menu-link": true,
                active: active === i,
              }),
              onClick: () => setOpen(false),
              onFocus: () => setOpen(true),
            };
            return path === location.pathname ? (
              <a href={to} {...props}>
                {title}
              </a>
            ) : (
              <Link to={to} {...props}>
                {title}
              </Link>
            );
          }
        )}
      </nav>
    </>
  );
};

export default Menu;
