import LukeWorks from "LukeWorks";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import reportWebVitals from "reportWebVitals";

const container = document.getElementById("app");
if (container) {
  const root = createRoot(container);
  root.render(
    <StrictMode>
      <LukeWorks />
    </StrictMode>
  );
}

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
