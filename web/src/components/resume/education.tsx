import s from "pages/styles/resume.module.css";

export type EducationInfo = {
  start: string;
  end?: string;
  title: string;
  entity: string;
  location: string;
  icon: string;
  bullets: string[];
};

export type EducationProps = {
  education: EducationInfo;
};

export default function Education({ education }: EducationProps) {
  return (
    <section className={s.education}>
      <h3>Education</h3>
      {education.degree}
      <br />
      {education.field}
      <br />
      {education.entity}
      <br />
      {education.start} - {education.end}
      <br />
      {education.location}
    </section>
  );
}
