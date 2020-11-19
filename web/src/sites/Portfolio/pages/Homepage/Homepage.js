import React from "react";

import Footer from "../../components/Footer";
import Header from "../../components/Header";
import About from "./sections/About";
import Blog from "./sections/Blog";
import Contact from "./sections/Contact";
import Intro from "./sections/Intro";
import Projects from "./sections/Projects";

const Homepage = () => (
  <>
    <Header />
    <main>
      <Intro />
      <Blog />
      <Projects />
      <About />
      <Contact />
    </main>
    <Footer />
  </>
);

export default Homepage;
