import clsx from "clsx";
import { MenuIcon } from "components/icons";
import LinkIcon from "components/linkIcon";
import routes from "data/routes.json";
import useClickOutside from "hooks/useClickOutside";
import useEventListener from "hooks/useEventListener";
import Link from "next/link";
import { useRouter } from "next/router";
import { useCallback, useEffect, useRef, useState } from "react";
import s from "./menu.module.css";

const hasOffset = (el: Element): el is HTMLElement => {
  return (el as HTMLElement).offsetTop !== undefined;
};

export default function Menu() {
  const router = useRouter();

  const [active, setActive] = useState(-1);
  const [open, setOpen] = useState(false);
  const scrollUpdateRef = useRef<ReturnType<typeof setTimeout>>();
  const menuRef = useRef<HTMLDivElement>(null);
  const menuIconRef = useRef<HTMLDivElement>(null);

  const toggleMenu = () => setOpen((open) => !open);
  const handleMenuClick = () => setOpen(false);
  const handleMenuFocus = () => setOpen(true);

  const scrollUpdate = useCallback(() => {
    if (router.isReady && router.pathname === routes.home.path) {
      const offsets = Array.from(document.querySelectorAll("span.anchor"))
        .map((el) => (hasOffset(el) ? el.offsetTop : Infinity))
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
    } else {
      setActive(-1);
    }
  }, [router.isReady, router.pathname]);

  useEffect(() => {
    menuRef.current?.style.setProperty(
      "height",
      open ? "var(--menu-height)" : "0",
    );
  }, [open]);

  useEffect(() => {
    scrollUpdate();
  }, [scrollUpdate]);

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
        <LinkIcon
          className={s.menuIcon}
          title="Navigation Menu"
          icon={MenuIcon}
          onClick={toggleMenu}
        />
      </div>
      <nav className={s.menu} ref={menuRef}>
        {Object.values(routes.menu).map(({ hash, title }, i) => (
          <Link key={hash} href={`/${hash}`}>
            <a
              className={clsx(s.menuLink, active === i && s.active)}
              onClick={handleMenuClick}
              onFocus={handleMenuFocus}
            >
              {title}
            </a>
          </Link>
        ))}
      </nav>
    </>
  );
}
