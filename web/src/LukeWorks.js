import GlobalStyles from "global.styles";
import PropTypes from "prop-types";
import React, { useEffect, useRef, useState } from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import { CSSTransition } from "react-transition-group";
import SitePaths from "sitePaths";
import { Portfolio, Resume } from "sites";

const FadeRoute = ({ children, ...props }) => {
  let [isLoaded, setIsLoaded] = useState(false);
  const fadeRef = useRef(null);

  useEffect(() => {
    setTimeout(() => setIsLoaded(true), 200);
  }, []);

  return (
    <Route {...props}>
      {(match) => (
        <CSSTransition
          nodeRef={fadeRef}
          in={isLoaded && match !== null}
          timeout={1000}
          classNames="fade"
          unmountOnExit
        >
          <div ref={fadeRef} className="fade">
            {children}
          </div>
        </CSSTransition>
      )}
    </Route>
  );
};

FadeRoute.propTypes = { children: PropTypes.node.isRequired };

const LukeWorks = () => {
  return (
    <Router>
      <GlobalStyles />
      <Switch>
        <Route exact path={SitePaths.resume}>
          <Resume />
        </Route>
        <FadeRoute>
          <Portfolio />
        </FadeRoute>
      </Switch>
    </Router>
  );
};

export default LukeWorks;
