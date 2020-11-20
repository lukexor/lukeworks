import PropTypes from "prop-types";
import React from "react";
import { Redirect, Route } from "react-router-dom";

import SitePaths from "../sitePaths";

// TODO: PrivateRoute
const PrivateRoute = ({ children, ...props }) => {
  const loggedInUser = null;

  return (
    <Route
      {...props}
      render={({ location }) =>
        loggedInUser ? (
          children
        ) : (
          <Redirect
            to={{ pathname: SitePaths.login, state: { from: location } }}
          />
        )
      }
    />
  );
};

PrivateRoute.propTypes = {
  children: PropTypes.node,
};

export default PrivateRoute;
