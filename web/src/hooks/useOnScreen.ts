import { RefObject, useEffect, useState } from "react";

export default function useOnScreen<T extends Element>(
  ref: RefObject<T>,
  rootMargin = "0px",
  root?: Element | Document,
): boolean {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry && entry.isIntersecting) {
          setIsVisible(entry.isIntersecting);
        }
      },
      { root: root ?? document, rootMargin, threshold: 0.1 },
    );

    const current = ref.current;
    current && observer.observe(current);
    return () => {
      current && observer.unobserve(current);
    };
  }, [ref, root, rootMargin]);

  return isVisible;
}
