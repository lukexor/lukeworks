import { RefObject, useEffect, useRef } from "react";

export function useEventListener<K extends keyof WindowEventMap>(
  eventName: K,
  handler: (evt: WindowEventMap[K]) => void,
): void;

export function useEventListener<
  K extends keyof HTMLElementEventMap,
  T extends HTMLElement = HTMLDivElement,
>(
  eventName: K,
  handler: (evt: HTMLElementEventMap[K]) => void,
  element: RefObject<T>,
): void;

// Credit: https://usehooks.com/useEventListener/
export function useEventListener<
  KW extends keyof WindowEventMap,
  KH extends keyof HTMLElementEventMap, // this extends DocumentEventMap
  T extends HTMLElement | void = void,
>(
  eventName: KW | KH,
  handler: (evt: WindowEventMap[KW] | HTMLElementEventMap[KH] | Event) => void,
  element?: RefObject<T>,
): void {
  const handlerRef = useRef(handler);

  useEffect(() => {
    handlerRef.current = handler;
  }, [handler]);

  useEffect(() => {
    const target = element?.current || window;
    const isSupported = target && target.addEventListener;
    if (!isSupported) {
      return;
    }

    const listener: typeof handler = (event) => handlerRef.current?.(event);

    target.addEventListener(eventName, listener);
    return () => {
      target.removeEventListener(eventName, listener);
    };
  }, [eventName, element, handler]);
}

export default useEventListener;
