import "./Homepage.css";
import About from "./sections/about";
import Blog from "./sections/blog";
import Contact from "./sections/contact";
import Intro from "./sections/intro";
import Projects from "./sections/projects";

const Homepage = () => {
  return (
    <main className="homepage">
      <Intro />
      <Blog />
      <Projects />
      <About />
      <Contact />
    </main>
  );
};

export default Homepage;
