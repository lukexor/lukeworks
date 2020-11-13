import bgImg from "code-bg.jpg";
import { createGlobalStyle } from "styled-components";

const GlobalStyles = createGlobalStyle`
  body::before {
    content: "''";
    background-image: url(${bgImg});
    background-repeat: no-repeat;
    background-size: cover;
    opacity: 0.05;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: -9999;
  }
  body {
    background-color: #212525;
    color: #d1e8e2;
    font-family: "Fira Sans";
    font-size: 1.2em;
    margin: 0;
  }

  .fade-enter {
    opacity: 0;
  }
  .fade-enter-active {
    opacity: 1;
    transition: opacity 1s ease-in 0.1s;
  }
  .fade-exit {
    opacity: 1;
  }
  .fade-exit-active {
    opacity: 0;
    transition: opacity 1s ease-out 0.1s;
  }
}
`;

export default GlobalStyles;
