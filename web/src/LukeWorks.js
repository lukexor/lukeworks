import GlobalStyles from "global.styles";
import React from "react";
import { HelmetProvider } from "react-helmet-async";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import SitePaths from "sitePaths";
import { Portfolio, Resume } from "sites";

import Admin, { Login } from "./sites/Admin";
import ErrorBoundary from "./util/ErrorBoundary";
import PrivateRoute from "./util/PrivateRoute";

const LukeWorks = () => (
  <HelmetProvider>
    <Router>
      <ErrorBoundary>
        <GlobalStyles />
        <Switch>
          <Route exact path={SitePaths.resume}>
            <Resume />
          </Route>
          <PrivateRoute path={SitePaths.admin}>
            <Admin />
          </PrivateRoute>
          <Route path={SitePaths.login}>
            <Login />
          </Route>
          <Route>
            <Portfolio />
          </Route>
        </Switch>
      </ErrorBoundary>
    </Router>
  </HelmetProvider>
);

export default LukeWorks;
