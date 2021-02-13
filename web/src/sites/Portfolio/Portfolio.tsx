import React, { useCallback, useEffect, useState } from "react";
import { Helmet } from "react-helmet-async";
import { Route, Switch } from "react-router-dom";
import { ThemeProvider } from "styled-components";
import useEventListener from "util/hooks/useEventListener";
import copy from "./data/copy.json";
import Homepage from "./pages/Homepage/Homepage";
import Post from "./pages/Post";
import { GlobalStyles } from "./portfolio.styles";
import routes from "./routes";
import theme from "./theme";

const Portfolio: React.FC = () => {
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    setTimeout(() => {
      setIsLoaded(true);
    }, 200);
  }, []);

  // Required because of HashLink using ids without a hash
  const scrollIntoView = useCallback(() => {
    const hash = window.location.hash.substring(1);
    if (!hash) {
      return;
    }
    const el = document.getElementById(hash);
    if (el) {
      const yCoordinate = el.offsetTop;
      window.scrollTo({ top: yCoordinate, behavior: "smooth" });
    }
  }, []);

  useEventListener("load", scrollIntoView);

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
            <Route exact path={routes.post.path}>
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
