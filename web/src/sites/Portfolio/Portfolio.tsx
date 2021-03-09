import React, { useEffect, useState } from "react";
import { Helmet } from "react-helmet-async";
import { Redirect, Route, Switch, useLocation } from "react-router-dom";
import { ThemeProvider } from "styled-components";
import Footer from "./components/Footer";
import Header from "./components/Header";
import copy from "./data/copy.json";
import legacyRoutes from "./legacyRoutes";
import Homepage from "./pages/Homepage/Homepage";
import Post from "./pages/Post";
import { GlobalStyles } from "./portfolio.styles";
import routes from "./routes.json";
import theme from "./theme";

const Portfolio: React.FC = () => {
  const [isLoaded, setIsLoaded] = useState(false);
  const { pathname } = useLocation();

  useEffect(() => {
    setTimeout(() => {
      setIsLoaded(true);
    }, 100);
  }, []);

  useEffect(() => {
    window.scrollTo(0, 0);

    // Required because of HashLink using ids without a hash
    const scrollIntoView = () => {
      const hash = window.location.hash.substring(1);
      if (!hash) {
        return;
      }
      const el = document.getElementById(hash);
      if (el) {
        window.scrollTo({ top: el.offsetTop });
      }
    };

    setTimeout(scrollIntoView, 300);
  }, [pathname]);

  return (
    <>
      <Helmet>
        <title>{copy.Head.title}</title>
        <meta name="description" content={copy.Head.description} />
      </Helmet>

      <ThemeProvider theme={theme}>
        <GlobalStyles />
        <div className={`fade-enter ${isLoaded ? "fade-enter-active" : ""}`}>
          <Header />
          <Switch>
            {/* TODO handle search */}
            {Object.entries(legacyRoutes).map(([legacyRoute, newRoute]) => (
              <Route key={legacyRoute} exact path={legacyRoute}>
                <Redirect to={newRoute} />
              </Route>
            ))}
            <Route exact path={routes.post}>
              <Post />
            </Route>
            {/* TODO handle 404 responses */}
            <Route>
              <Homepage />
            </Route>
          </Switch>
          <Footer />
        </div>
      </ThemeProvider>
    </>
  );
};

export default Portfolio;
