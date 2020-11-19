import React, { useEffect, useState } from "react";
import theme from "sites/Portfolio/theme";

import { Bg, BgGlitch,StyledSplashBg } from "./splashBg.styles";

const SplashBg = () => {
  const [glitch, setGlitch] = useState(false);
  const [color, setColor] = useState(theme.colors.accentDark);
  const [direction, setDirection] = useState(1);

  useEffect(() => {
    const glitchTimer = setInterval(() => {
      if (Math.random() < 0.3) {
        Math.random() < 0.5
          ? setColor(theme.colors.accentDark)
          : setColor(theme.colors.secondary);
        Math.random() < 0.5 ? setDirection(-1) : setDirection(1);
        setGlitch(true);

        setTimeout(() => {
          setGlitch(false);
        }, 300);
      }
    }, 2000);

    return () => {
      clearInterval(glitchTimer);
    };
  }, []);

  return (
    <StyledSplashBg>
      <Bg />
      <BgGlitch glitch={glitch} bgColor={color} direction={direction} />
    </StyledSplashBg>
  );
};

export default SplashBg;
