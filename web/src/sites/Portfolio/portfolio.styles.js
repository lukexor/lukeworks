import { createGlobalStyle } from "styled-components";

const GlobalStyles = createGlobalStyle`
  body {
    background-color: ${(props) => props.theme.colors.background};
    color: ${(props) => props.theme.colors.primary};
    font-family: ${(props) => props.theme.fontSans};
    font-size: 1.2em;
    margin: 0;
  }
  h1 {
    color: ${(props) => props.theme.colors.primary};
    font-family: ${(props) => props.theme.fontSerif};
    font-size: ${(props) => props.theme.sizes.xlarge};
    font-weight: normal;
    text-align: center;

    @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
      font-size: ${(props) => props.theme.sizes.xxlarge};
    }
  }
  h2 {
    font-family: ${(props) => props.theme.fontSerif};
    font-size: ${(props) => props.theme.sizes.large};
    font-weight: normal;
    line-height: 1.6em;
    text-align: center;

    @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
      font-size: ${(props) => props.theme.sizes.xlarge};
    }
  }

  main a.anchor {
    position: relative;
    top: ${() => `-${document.querySelector("a#home").offsetTop}px`};
    display: block;
    visibility: hidden;
  }
}
`;

export { GlobalStyles };
