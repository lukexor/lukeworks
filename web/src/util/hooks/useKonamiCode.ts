import { useEffect, useState } from "react";

type KeyMap = {
  [key: string]: string;
};

const KEYS: KeyMap = {
  37: "left",
  38: "up",
  39: "right",
  40: "down",
  65: "A",
  66: "B",
};

const getKeyName = (code: string) => {
  return KEYS[code];
};

const useKonamiCode = (handler: () => void): boolean => {
  const [keys, setKeys] = useState<string[]>([]);

  const isKonamiCode =
    keys.join(" ") === "up up down down left right left right B A";

  useEffect(() => {
    let timeout: NodeJS.Timeout;

    document.onkeydown = (evt: KeyboardEvent) => {
      setKeys((currentKeys) => [...currentKeys, getKeyName(evt.code)]);
      timeout && clearTimeout(timeout);
      timeout = setTimeout(() => setKeys([]), 5000);
    };
  }, []);

  useEffect(() => {
    if (isKonamiCode) {
      handler();
      setKeys([]);
    }
  }, [isKonamiCode, handler]);

  return isKonamiCode;
};

export default useKonamiCode;
