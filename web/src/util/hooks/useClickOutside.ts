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
      if (!el || el.contains((event.target as Node) || null)) {
        return;
      }

      handler(event);
    };

    document.addEventListener("mousedown", listener);
    document.addEventListener("touchstart", listener);

    return () => {
      document.removeEventListener("mousedown", listener);
      document.removeEventListener("touchstart", listener);
    };
  }, [ref, handler]);
};

export default useClickOutside;
