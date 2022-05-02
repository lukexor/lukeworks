import { RefObject, useEffect, useRef, useState } from "react";

type UseOnScreenResult<T> = [RefObject<T>, boolean];

const useOnScreen = <T extends Element>(
  rootMargin = "0px",
  root = document
): UseOnScreenResult<T> => {
  const [isVisible, setIsVisible] = useState(false);
  const ref = useRef<T>(null);

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
  }, []);

  return [ref, isVisible];
};

export default useOnScreen;
