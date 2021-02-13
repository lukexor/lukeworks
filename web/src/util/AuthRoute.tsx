import PropTypes from "prop-types";
import React from "react";
import { Redirect, Route, RouteProps } from "react-router-dom";
import routes from "routes.json";

// TODO: AuthRoute
// Set up useAuth hook with AuthProvider
const AuthRoute: React.FC<RouteProps> = ({ children, ...props }) => {
  const loggedInUser = null;

  return (
    <Route
      {...props}
      render={({ location }) =>
        loggedInUser ? (
          children
        ) : (
          <Redirect
            to={{ pathname: routes.login.path, state: { from: location } }}
          />
        )
      }
    />
  );
};

AuthRoute.propTypes = {
  children: PropTypes.node,
};

export default AuthRoute;
