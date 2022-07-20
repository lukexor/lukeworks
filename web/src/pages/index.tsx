import About from "components/homepage/about";
import Blog from "components/homepage/blog";
import Contact from "components/homepage/contact";
import Intro from "components/homepage/intro";
import Projects from "components/homepage/projects";
import Layout from "components/layout";
import s from "./styles/index.module.css";

export default function Lukeworks() {
  return (
    <Layout>
      <main className={s.main}>
        <Intro />
        <Blog />
        <Projects />
        <About />
        <Contact />
      </main>
    </Layout>
  );
}
