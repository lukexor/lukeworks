import Header from "../components/Header";
import React from "react";
import Splash from "../components/Splash";
import styled from "styled-components";

const StyledLanding = styled.section`
  display: flex;
  flex-direction: column;
  min-height: 100vh;
`;

const Landing = () => (
  <StyledLanding>
    <Header />
    <Splash />
  </StyledLanding>
);

export default Landing;
