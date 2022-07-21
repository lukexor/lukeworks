import { RefObject, useEffect, useState } from "react";

export default function useOnScreen<T extends Element>(
  ref: RefObject<T>,
  rootMargin = "0px",
  root: null | HTMLElement = null,
): boolean {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry && entry.isIntersecting) {
          setIsVisible(entry.isIntersecting);
        }
      },
      // `document` is not supported as root on some browsers
      { root: root, rootMargin, threshold: 0.1 },
    );

    const current = ref.current;
    current && observer.observe(current);
    return () => {
      current && observer.unobserve(current);
    };
  }, [ref, root, rootMargin]);

  return isVisible;
}
