import { RefObject, useEffect, useState } from "react";

export default function useOnScreen<T extends Element>(
  ref: RefObject<T>,
  rootMargin = "0px",
  root = document
): boolean {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry && entry.isIntersecting) {
          setIsVisible(entry.isIntersecting);
        }
      },
      { root, rootMargin }
    );

    const current = ref.current;
    current && observer.observe(current);
    return () => {
      current && observer.unobserve(current);
    };
  }, [ref, root, rootMargin]);

  return isVisible;
}
