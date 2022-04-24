import useMetaTag from "hooks/useMetaTag";
import { useEffect, useState } from "react";
import Education, { EducationInfo } from "./components/Education";
import Employment, { EmploymentInfo } from "./components/Employment";
import Experience, { ExperienceInfo } from "./components/Experience";
import Header, { HeaderInfo } from "./components/Header";
import Profile, { ProfileInfo } from "./components/Profile";
import Skills from "./components/Skills";

type ResumeInfo = {
  title: string;
  description: string;
  header: HeaderInfo;
  profile: ProfileInfo;
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
    <>
      <Header header={resume.header} />
      <main>
        <Profile profile={resume.profile} />
        <Skills list={resume.langAndTech} />
        <section>
          <Employment list={resume.employment} />
          <Education list={resume.education} />
          <Experience list={resume.experience} />
        </section>
      </main>
    </>
  );
};

export default Resume;
