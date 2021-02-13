import { createGlobalStyle } from "styled-components";

const GlobalStyles = createGlobalStyle`
  body {
    background-color: ${(props) => props.theme.colors.background};
    -webkit-font-smoothing: antialiased;
    color: ${(props) => props.theme.colors.primary};
    font-family: ${(props) => props.theme.fontSans};
    font-size: 1.2em;
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

  main span.anchor {
    top: -${(props) => props.theme.sizes.xxlarge};
    position: relative;
    display: block;
  }

  @media only screen (min-width: ${(props) =>
    props.theme.breakpoints.desktopSmall}) {
    body {
      scrollbar-width: thin;
      scrollbar-color: ${(props) =>
        `${props.theme.colors.accentDark} ${props.theme.colors.backgroundLight}`}
      scroll-behavior: smooth;
      overflow: hidden auto;
    }
    body::-webkit-scrollbar {
      width: ${(props) => props.theme.sizes.medSmall};
    }
    body::-webkit-scrollbar-thumb {
      background: ${(props) => props.theme.colors.accentDark};
      border-radius: 6px;
      border: 3px solid ${(props) => props.theme.colors.backgroundLight};
      -webkit-border-radius: 10px;
    }
    body::-webkit-scrollbar-track {
      background: ${(props) => props.theme.colors.backgroundLight};
      -webkit-box-shadow: inset 0 0 6px rgba(0,0,0,0.3);
    }
  }
}
`;

export { GlobalStyles };
