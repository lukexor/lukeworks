import bgImg from "code-bg.jpg";
import styled from "styled-components";

const StyledSplashBg = styled.div`
  &::before {
    content: "";
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
    z-index: -2000;
  }

  &::after {
    content: "";
    opacity: ${(props) => (props.glitch ? "50%" : "0")};
    background-color: ${(props) => props.bgColor};
    background-image: url(${bgImg});
    background-repeat: repeat;
    background-size: cover;
    background-blend-mode: luminosity;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: translateX(${(props) => `${props.direction * 30}px`});
    z-index: -2000;
  }
`;

export { StyledSplashBg };
