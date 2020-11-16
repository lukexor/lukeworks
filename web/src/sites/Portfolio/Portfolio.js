import React, { useEffect, useState } from "react";
import { Route, Switch } from "react-router-dom";
import SitePaths from "sitePaths";
import { ThemeProvider } from "styled-components";

import Homepage from "./pages/Homepage/Homepage";
import Post from "./pages/Post";
import { GlobalStyles } from "./portfolio.styles";
import theme from "./theme";

const Portfolio = () => {
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    setTimeout(() => {
      setIsLoaded(true);
    }, 200);

    // Required because of HashLink using ids without a hash
    const scrollIntoView = () => {
      const hash = window.location.hash.substring(1);
      if (!hash) {
        return;
      }
      const el = document.getElementById(hash);
      if (el) {
        const yCoordinate = el.offsetTop;
        window.scrollTo({ top: yCoordinate });
      }
    };

    window.addEventListener("load", scrollIntoView);
    return () => {
      window.removeEventListener("load", scrollIntoView);
    };
  }, []);

  return (
    <ThemeProvider theme={theme}>
      <GlobalStyles />
      <div className={`fade-enter ${isLoaded ? "fade-enter-active" : ""}`}>
        <Switch>
          <Route exact path={SitePaths.post}>
            <Post />
          </Route>
          <Route>
            <Homepage />
          </Route>
        </Switch>
      </div>
    </ThemeProvider>
  );
};

export default Portfolio;
