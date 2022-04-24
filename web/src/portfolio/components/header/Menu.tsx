// import { useCallback, useRef, useState } from "react";
// import copy from "portfolio/data/copy.json";
// import Icons from "portfolio/Icons";
// import routes from "portfolio/routes.json";
// import useClickOutside from "hooks/useClickOutside";
// import useEventListener from "hooks/useEventListener";

// type Route = keyof typeof routes;
// type MenuItem = {
//   url: Route;
//   title: string;
// };

// const routeLinks: MenuItem[] = copy.Menu.list.map((item) => ({
//   title: item,
//   url: item.toLowerCase() as Route,
// }));

// const Menu = () => {
//   const [active, setActive] = useState(0);
//   const [open, setOpen] = useState(false);
//   const toggleOpen = () => setOpen((open) => !open);
//   const menuRef = useRef<HTMLDivElement>(null);
//   const menuIconRef = useRef<HTMLDivElement>(null);

//   const updateActive = useCallback(() => {
//     const offsets = Array.from(document.querySelectorAll("span.anchor"))
//       .concat(document.querySelector("section#splash") || [])
//       .map((el) => (el as HTMLElement).offsetTop)
//       .sort((a, b) => a - b);

//     let closest = active;
//     let closestDiff = Infinity;
//     offsets.forEach((offset, i) => {
//       const diff = Math.abs(window.scrollY - offset);
//       if (diff < closestDiff) {
//         closestDiff = diff;
//         closest = i;
//       }
//     });
//     setActive(closest);
//   }, []);

//   useEventListener("scroll", updateActive);

//   useClickOutside(menuRef, (event) => {
//     const icon = menuIconRef.current;
//     const target = event.target as Node;
//     if (!icon || (icon && !icon.contains(target))) {
//       setOpen(false);
//     }
//   });

//   return (
//     <>
//       <div ref={menuIconRef}>
//         <StyledMenuIcon icon={Icons.menu} onClick={toggleOpen} />
//       </div>
//       <StyledMenu visible={open} ref={menuRef}>
//         {routeLinks.map(({ url, title }, i) => (
//           <MenuLink
//             key={url}
//             to={routes[url]}
//             active={active === i}
//             onClick={() => setOpen(false)}
//             onFocus={() => setOpen(true)}
//             smooth
//           >
//             {title}
//           </MenuLink>
//         ))}
//       </StyledMenu>
//     </>
//   );
// };

const Menu = () => {
  return <></>;
};

export default Menu;
