import { Lukeworks, Resume, TetanesWeb } from "sites";
import React, { useEffect, useRef, useState } from "react";
import { Route, BrowserRouter as Router, Switch } from "react-router-dom";
import { CSSTransition } from "react-transition-group";
import GlobalStyles from "global.styles";
import ReactDOM from "react-dom";
import SitePaths from "sitePaths";
import reportWebVitals from "reportWebVitals";

const App = () => {
  let [isLoaded, setIsLoaded] = useState(false);
  const nodeRef = useRef(null);

  useEffect(() => {
    setTimeout(() => setIsLoaded(true), 200);
  }, []);

  return (
    <Router>
      <Switch>
        <Route exact path={SitePaths.resume}>
          <Resume />
        </Route>
        <Route exact path={SitePaths.tetanes}>
          <TetanesWeb />
        </Route>
        <Route>
          {({ match }) => (
            <CSSTransition
              nodeRef={nodeRef}
              in={isLoaded && match !== null}
              timeout={1000}
              classNames="fade"
              unmountOnExit
            >
              <div ref={nodeRef} className="fade">
                <Lukeworks />
              </div>
            </CSSTransition>
          )}
        </Route>
      </Switch>
    </Router>
  );
};

ReactDOM.render(
  <React.StrictMode>
    <GlobalStyles />
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
