import "./Menu.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import classNames from "classnames";
import useClickOutside from "hooks/useClickOutside";
import useEventListener from "hooks/useEventListener";
import Icons from "portfolio/Icons";
import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
} from "react";
import { Link, useLocation } from "react-router-dom";
import routes from "routes.json";

const { home } = routes;

const Menu = () => {
  const [active, setActive] = useState(-1);
  const [open, setOpen] = useState(false);
  const scrollUpdateRef = useRef<ReturnType<typeof setTimeout>>();
  const menuRef = useRef<HTMLDivElement>(null);
  const menuIconRef = useRef<HTMLDivElement>(null);
  const toggleOpen = () => setOpen((open) => !open);
  const location = useLocation();

  useEffect(() => {
    const height = open ? "17.5ch" : "0";
    menuRef.current?.style.setProperty("--height", height);
  }, [open]);

  const scrollUpdate = useCallback(() => {
    if (location.pathname !== home.path) {
      setActive(-1);
      return;
    }

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
  }, [location.pathname]);

  useLayoutEffect(() => {
    scrollUpdate();
  }, [location]);

  useEventListener("scroll", () => {
    if (scrollUpdateRef.current) {
      clearTimeout(scrollUpdateRef.current);
    }
    scrollUpdateRef.current = setTimeout(scrollUpdate, 500);
  });

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
          className="link-icon"
          title="Navigation Menu"
          icon={Icons.menu}
          onClick={toggleOpen}
        />
      </div>
      <nav className="menu" ref={menuRef}>
        {Object.values(routes.menu).map(({ hash, title }, i) => {
          const props = {
            key: hash,
            className: classNames({
              "menu-link": true,
              active: active === i,
            }),
            onClick: () => setOpen(false),
            onFocus: () => setOpen(true),
          };
          // Hashes on the same page can't use react-router
          return location.pathname === home.path ? (
            <a href={hash} {...props}>
              {title}
            </a>
          ) : (
            <Link to={`/${hash}`} {...props}>
              {title}
            </Link>
          );
        })}
      </nav>
    </>
  );
};

export default Menu;
