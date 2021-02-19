import { createGlobalStyle } from "styled-components";

const GlobalStyles = createGlobalStyle`
  body {
    background-color: ${(props) => props.theme.colors.background};
    -webkit-font-smoothing: antialiased;
    color: ${(props) => props.theme.colors.secondary};
    font-family: ${(props) => props.theme.fontSans};
    font-size: 1.2em;
  }
  h1, h2, h3, h4, h5, h6 {
    color: ${(props) => props.theme.colors.primary};
    font-family: ${(props) => props.theme.fontSerif};
    font-weight: normal;
    text-align: center;
  }
  h1 {
    font-size: ${(props) => props.theme.sizes.xxlarge};
    @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
      font-size: ${(props) => props.theme.sizes.xxxlarge};
    }
  }
  h2 {
    font-size: ${(props) => props.theme.sizes.xlarge};
    @media (min-width: ${(props) => props.theme.breakpoints.tablet}) {
      font-size: ${(props) => props.theme.sizes.xxlarge};
    }
  }

  a:not(.img) {
    color: ${(props) => props.theme.colors.accentLight};
    text-decoration: none;
    transition: color 0.5s ease;

    &:hover {
      color: ${(props) => props.theme.colors.accentDark};
    }
  }

  main span.anchor {
    top: -${(props) => props.theme.sizes.xxxlarge};
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
