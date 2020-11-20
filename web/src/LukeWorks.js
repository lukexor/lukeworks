import GlobalStyles from "global.styles";
import React from "react";
import { HelmetProvider } from "react-helmet-async";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import SitePaths from "sitePaths";
import { Portfolio, Resume } from "sites";

const LukeWorks = () => (
  <HelmetProvider>
    <Router>
      <GlobalStyles />
      <Switch>
        <Route exact path={SitePaths.resume}>
          <Resume />
        </Route>
        <Route>
          <Portfolio />
        </Route>
      </Switch>
    </Router>
  </HelmetProvider>
);

export default LukeWorks;
