import React from "react";
import Header from "sites/Portfolio/components/Header";

import About from "./components/About";
import Blog from "./components/Blog";
import Contact from "./components/Contact";
import Landing from "./components/Landing";
import Projects from "./components/Projects";

const Homepage = () => (
  <>
    <Header />
    <Landing />
    <Blog />
    <Projects />
    <About />
    <Contact />
  </>
);

export default Homepage;
