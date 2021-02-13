import GlobalStyles from "global.styles";
import React from "react";
import { HelmetProvider } from "react-helmet-async";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import SitePaths from "sitePaths";
import { Portfolio, Resume } from "sites";
import Admin, { Login } from "sites/Admin";
import AuthRoute from "util/AuthRoute";
import ErrorBoundary from "util/ErrorBoundary";

const LukeWorks: React.FC = () => (
  <HelmetProvider>
    <Router>
      <ErrorBoundary>
        <GlobalStyles />
        <Switch>
          <Route exact path={SitePaths.resume}>
            <Resume />
          </Route>
          <AuthRoute path={SitePaths.admin}>
            <Admin />
          </AuthRoute>
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
