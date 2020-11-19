import { createGlobalStyle } from "styled-components";

const GlobalStyles = createGlobalStyle`
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
