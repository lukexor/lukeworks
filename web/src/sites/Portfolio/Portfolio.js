import React, { useEffect, useState } from "react";
import { Helmet } from "react-helmet-async";
import { Route, Switch } from "react-router-dom";
import SitePaths from "sitePaths";
import { copy } from "sites/Portfolio/util/constants";
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
    <>
      <Helmet>
        <title>{copy.Head.title}</title>
        <meta name="description" content={copy.Head.description} />
      </Helmet>

      <ThemeProvider theme={theme}>
        <GlobalStyles />
        <div className={`fade-enter ${isLoaded ? "fade-enter-active" : ""}`}>
          <Switch>
            <Route exact path={SitePaths.post}>
              <Post />
            </Route>
            {/* TODO handle 404 responses */}
            <Route>
              <Homepage />
            </Route>
          </Switch>
        </div>
      </ThemeProvider>
    </>
  );
};

export default Portfolio;
