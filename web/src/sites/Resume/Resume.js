import React from "react";
import { Helmet } from "react-helmet-async";

import Education from "./components/Education";
import Employment from "./components/Employment";
import Experience from "./components/Experience";
import Header from "./components/Header";
import Profile from "./components/Profile";
import Skills from "./components/Skills";
import resumeInfo from "./resumeInfo";

const Resume = () => (
  <>
    <Helmet>
      <title>{resumeInfo.title}</title>
      <meta name="description" content={resumeInfo.description} />
    </Helmet>

    <Header {...resumeInfo.header} />

    <main>
      <Profile {...resumeInfo.profile} />
      <Skills list={resumeInfo.langAndTech} />
      <section>
        <Employment list={resumeInfo.employment} />
        <Education list={resumeInfo.education} />
        <Experience list={resumeInfo.experience} />
      </section>
    </main>
  </>
);

export default Resume;
