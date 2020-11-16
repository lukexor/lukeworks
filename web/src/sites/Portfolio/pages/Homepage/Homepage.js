import React from "react";
import Footer from "sites/Portfolio/components/Footer";
import Header from "sites/Portfolio/components/Header";

import About from "./components/About";
import Blog from "./components/Blog";
import Contact from "./components/Contact";
import Landing from "./components/Landing";
import Projects from "./components/Projects";

const Homepage = () => (
  <>
    <Header />
    <main>
      <Landing />
      <Blog />
      <Projects />
      <About />
      <Contact />
    </main>
    <Footer />
  </>
);

export default Homepage;
