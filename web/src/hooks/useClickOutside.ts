import { RefObject, useEffect } from "react";

type ClickEvent = MouseEvent | TouchEvent;

type ClickHandler = (evt: ClickEvent) => void;

const useClickOutside = <T extends HTMLElement = HTMLElement>(
  ref: RefObject<T>,
  handler: ClickHandler
): void => {
  useEffect(() => {
    const listener = (event: ClickEvent) => {
      // Do nothing if clicking ref's element or descendent elements
      const el = ref.current;
      const target = event.target as Node;
      if (el && !el.contains(target)) {
        handler(event);
      }
    };

    document.addEventListener("mouseup", listener);
    document.addEventListener("touchend", listener);

    return () => {
      document.removeEventListener("mouseup", listener);
      document.removeEventListener("touchend", listener);
    };
  }, [ref, handler]);
};

export default useClickOutside;
