import { RefObject, useEffect, useRef } from "react";

type EventHandler = (event: Event) => void;

// Credit: https://usehooks.com/useEventListener/
const useEventListener = <T extends HTMLElement = HTMLDivElement>(
  eventName: string,
  handler: EventHandler,
  element?: RefObject<T>
): void => {
  const savedHandler = useRef<EventHandler>();

  useEffect(() => {
    const targetElement: T | Window = element?.current || window;
    if (!targetElement || !targetElement.addEventListener) {
      return;
    }

    if (savedHandler.current !== handler) {
      savedHandler.current = handler;
    }

    const eventListener = (event: Event) => {
      if (savedHandler?.current) {
        savedHandler.current(event);
      }
    };

    targetElement.addEventListener(eventName, eventListener);
    return () => {
      targetElement.removeEventListener(eventName, eventListener);
    };
  }, [eventName, element, handler]);
};

export default useEventListener;
