import "./Resume.css";
import useMetaTag from "hooks/useMetaTag";
import { useEffect, useState } from "react";
import Contact, { ContactInfo } from "./components/Contact";
import Education, { EducationInfo } from "./components/Education";
import Employment, { EmploymentInfo } from "./components/Employment";
import Experience, { ExperienceInfo } from "./components/Experience";
import Header, { HeaderInfo } from "./components/Header";
import Technologies from "./components/Technologies";

type ResumeInfo = {
  title: string;
  description: string;
  header: HeaderInfo;
  contact: ContactInfo;
  employment: EmploymentInfo[];
  education: EducationInfo[];
  experience: ExperienceInfo[];
  addtlExperience: ExperienceInfo[];
  langAndTech: string[];
};

const Resume = () => {
  const [resume, setResume] = useState<ResumeInfo>();
  useMetaTag({ title: resume?.title, description: resume?.description });

  useEffect(() => {
    // TODO: Convert to API call
    import("./resumeInfo.json").then((info) => setResume(info.default));
  }, []);

  if (!resume) {
    return null;
  }

  return (
    <div className="resume">
      <Header header={resume.header} />
      <main className="content">
        <aside>
          <Contact contact={resume.contact} />
          <Technologies list={resume.langAndTech} />
        </aside>
        <section className="main-content">
          <Employment list={resume.employment} />
          <Education list={resume.education} />
          <Experience list={resume.experience} />
        </section>
      </main>
    </div>
  );
};

export default Resume;
