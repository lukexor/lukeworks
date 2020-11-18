import { useEffect, useState } from "react";

const useDebounce = (value, delay) => {
  const [debouncedValue, setDebouncedValue] = useState(null);

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

const useDebounceCallback = (func, delay) => {
  const [timer, setTimer] = useState(null);
  return () => {
    clearTimeout(timer);
    setTimer(
      setTimeout(() => {
        setTimer(null);
        func();
      }, delay)
    );
  };
};

export { useDebounce, useDebounceCallback };
