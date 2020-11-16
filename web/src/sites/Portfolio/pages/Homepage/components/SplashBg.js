import React, { useEffect, useState } from "react";

import { StyledSplashBg } from "./splashBg.styles";

const SplashBg = () => {
  const [glitch, setGlitch] = useState(false);

  useEffect(() => {
    const glitchTimer = setInterval(() => {
      const chance = Math.random();
      if (chance < 0.3) {
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

  return <StyledSplashBg glitch={glitch} />;
};

export default SplashBg;
