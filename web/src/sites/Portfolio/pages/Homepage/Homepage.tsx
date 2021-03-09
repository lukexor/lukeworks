import React from "react";
import { StyledHomepage } from "./homepage.styles";
import About from "./sections/About";
import Blog from "./sections/Blog";
import Contact from "./sections/Contact";
import Intro from "./sections/Intro";
import Projects from "./sections/Projects";

const Homepage: React.FC = () => (
  <StyledHomepage>
    <Intro />
    <Blog />
    <Projects />
    <About />
    <Contact />
  </StyledHomepage>
);

export default Homepage;
