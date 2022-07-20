import Contact, { ContactInfo } from "components/resume/contact";
import Education, { EducationInfo } from "components/resume/education";
import Employment, { EmploymentInfo } from "components/resume/employment";
import Experience, { ExperienceInfo } from "components/resume/experience";
import Header, { HeaderInfo } from "components/resume/header";
import Technologies from "components/resume/technologies";
import resume from "data/resumeInfo.json";
import Head from "next/head";
import s from "./styles/resume.module.css";

export type ResumeInfo = {
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

export default function Resume() {
  return (
    <>
      <Head>
        <title>{resume?.title}</title>
        <meta name="description" content={resume?.description} />
        <meta property="og:title" content={resume?.title} key="title" />
        <meta
          property="og:description"
          content={resume?.description}
          key="description"
        />
      </Head>
      <div className={s.resume}>
        <Header header={resume.header} />
        <main className={s.main}>
          <aside>
            <Contact contact={resume.contact} />
            <Technologies list={resume.langAndTech} />
          </aside>
          <section className={s.content}>
            <Employment list={resume.employment} />
            <Education list={resume.education} />
            <Experience list={resume.experience} />
          </section>
        </main>
      </div>
    </>
  );
}
