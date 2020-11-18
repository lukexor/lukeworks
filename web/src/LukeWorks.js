import GlobalStyles from "global.styles";
import React from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import SitePaths from "sitePaths";
import { Portfolio, Resume } from "sites";

const LukeWorks = () => (
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
);

export default LukeWorks;
