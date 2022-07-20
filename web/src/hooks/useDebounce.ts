import { useEffect, useState } from "react";

// Credit: https://usehooks.com/useDebounce/
export function useDebounce<T>(value: T, delay = 500): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedValue(value);
    }, delay);
    return () => {
      clearTimeout(timer);
    };
  }, [value, delay]);

  return debouncedValue;
}

export function useDebounceFn<T>(func: () => T, delay = 500): () => void {
  const [timer, setTimer] = useState<null | ReturnType<typeof setTimeout>>(
    null,
  );

  return () => {
    timer && clearTimeout(timer);
    setTimer(
      setTimeout(() => {
        setTimer(null);
        func();
      }, delay),
    );
  };
}
