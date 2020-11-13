import { Route, Switch } from "react-router-dom";
import Landing from "./pages/Landing";
import React from "react";
import SitePaths from "sitePaths";
import { ThemeProvider } from "styled-components";
import theme from "./theme";

const Blog = () => null;
const Project = () => null;

const Lukeworks = () => (
  <ThemeProvider theme={theme}>
    <Switch>
      <Route exact path={SitePaths.blog}>
        <Blog />
      </Route>
      <Route exact path={SitePaths.project}>
        <Project />
      </Route>
      <Route>
        <Landing />
      </Route>
    </Switch>
  </ThemeProvider>
);

export default Lukeworks;
