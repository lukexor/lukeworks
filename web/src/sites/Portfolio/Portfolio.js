import React, { useEffect, useState } from "react";
import { Route, Switch } from "react-router-dom";
import SitePaths from "sitePaths";
import { ThemeProvider } from "styled-components";

import Homepage from "./pages/Homepage/Homepage";
import Post from "./pages/Post";
import { GlobalStyles } from "./portfolio.styles";
import theme from "./theme";

const Portfolio = () => {
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

  return (
    <ThemeProvider theme={theme}>
      <GlobalStyles glitch={glitch} />
      <Switch>
        <Route exact path={SitePaths.post}>
          <Post />
        </Route>
        <Route>
          <Homepage />
        </Route>
      </Switch>
    </ThemeProvider>
  );
};

export default Portfolio;
