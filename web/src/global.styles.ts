import { createGlobalStyle } from "styled-components";

const GlobalStyles = createGlobalStyle`
  *:not(svg) {
    box-sizing: border-box;
  }
  html, body {
    margin: 0;
    padding: 0;
  }
  body {
    background: #000;
  }
  a {
    user-select: none;
  }
  p {
    white-space: pre-line;
    max-width: 70ch;
    margin: 25px auto;
  }

  .fade-enter {
    opacity: 0;
    transition: opacity 1s ease-in 0.1s;
  }
  .fade-enter-active {
    opacity: 1;
  }
}
`;

export default GlobalStyles;
