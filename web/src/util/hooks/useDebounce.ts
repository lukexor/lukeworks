import { useEffect, useState } from "react";

// Credit: https://usehooks.com/useDebounce/
const useDebounce = <T>(value: T, delay = 500): T => {
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
};

const useDebounceFn = <T>(func: () => T, delay = 500): (() => void) => {
  const [timer, setTimer] = useState<Maybe<NodeJS.Timeout>>(null);

  return () => {
    timer && clearTimeout(timer);
    setTimer(
      setTimeout(() => {
        setTimer(null);
        func();
      }, delay)
    );
  };
};

export { useDebounce, useDebounceFn };
