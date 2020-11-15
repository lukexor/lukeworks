import bgImg from "code-bg.jpg";
import { createGlobalStyle } from "styled-components";

const GlobalStyles = createGlobalStyle`
  body::before {
    content: "''";
    opacity: 20%;
    background-color: ${(props) => props.theme.colors.primary};
    background-image: url(${bgImg});
    background-repeat: repeat;
    background-size: cover;
    background-blend-mode: hard-light;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: -9999;
  }
  body::after {
    content: "''";
    opacity: ${(props) => (props.glitch ? "50%" : "0")};
    background-color: ${(props) =>
      Math.random() < 0.5
        ? props.theme.colors.accentDark
        : props.theme.colors.secondary};
    background-image: url(${bgImg});
    background-repeat: repeat;
    background-size: cover;
    background-blend-mode: luminosity;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: translateX(${() => Math.floor(Math.random() * 60 - 30) + "px"});
    z-index: -9999;
  }
  body {
    background-color: ${(props) => props.theme.colors.background};
    color: ${(props) => props.theme.colors.primary};
    font-family: "Fira Sans";
    font-size: 1.2em;
    margin: 0;
  }
}
`;

export { GlobalStyles };
