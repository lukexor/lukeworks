import GlobalStyles from "global.styles";
import React from "react";
import { HelmetProvider } from "react-helmet-async";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import routes from "routes.json";
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
          <Route exact path={routes.resume.path}>
            <Resume />
          </Route>
          <AuthRoute path={routes.admin.path}>
            <Admin />
          </AuthRoute>
          <Route path={routes.login.path}>
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
