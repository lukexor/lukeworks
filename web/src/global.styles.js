import { createGlobalStyle } from "styled-components";

const GlobalStyles = createGlobalStyle`
  a {
    user-select: none;
  }
  .fade-enter {
    opacity: 0;
    transition: opacity 1s ease-in 0.1s;
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
